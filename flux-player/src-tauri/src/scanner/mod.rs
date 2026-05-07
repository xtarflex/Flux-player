pub mod crawler;
pub mod metadata;
pub mod tmdb;

pub use metadata::MediaMetadata;
use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{AppHandle, Emitter, Runtime};

use std::collections::HashSet;

pub async fn scan_directory<R: Runtime>(app: AppHandle<R>, dir_path: String) -> Vec<MediaMetadata> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let mut existing_paths = HashSet::new();
    if let Ok(db_path) = crate::database::connection::get_db_path(&app) {
        if let Ok(conn) = rusqlite::Connection::open(db_path) {
            if let Ok(mut stmt) = conn.prepare("SELECT path FROM media") {
                if let Ok(mut rows) = stmt.query([]) {
                    while let Ok(Some(row)) = rows.next() {
                        if let Ok(p) = row.get::<_, String>(0) {
                            existing_paths.insert(p);
                        }
                    }
                }
            }
        }
    }

    let paths = crawler::walk_directory(&dir_path);
    let mut new_paths = Vec::new();

    for path in paths {
        let path_str = path.to_string_lossy().to_string();
        if !existing_paths.contains(&path_str) {
            new_paths.push(path);
        }
    }

    let total_new = new_paths.len();
    if total_new == 0 {
        return Vec::new();
    }

    // --- STAGE 1: Fast Pass (Skeletons) ---
    // Create skeleton metadata for all new files and save immediately
    let mut skeletons = Vec::new();
    let skeleton_total = new_paths.len();

    for (index, path) in new_paths.iter().enumerate() {
        // Emit progress for Stage 1 (Skeleton creation)
        // We could emit a distinct event like "flux-scan-skeleton-progress"
        // but for now we'll just emit progress normally.
        let _ = app.emit("flux-scan-progress", (index + 1, skeleton_total));

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default()
            .to_string();

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase())
            .unwrap_or_default();

        let (cleaned_title, year, _extracted_series) = metadata::clean_media_title(&file_stem);

        let media_type = if metadata::is_video(&ext) {
            "video".to_string()
        } else if metadata::is_audio(&ext) {
            "audio".to_string()
        } else {
            continue; // Skip unknown
        };

        let skeleton = MediaMetadata {
            path: path.to_string_lossy().to_string(),
            title: cleaned_title,
            year,
            artist: None,
            album: None,
            poster_path: None,
            backdrop_path: None,
            album_art_path: None,
            duration: None, // Missing
            media_type,
            synopsis: None, // Missing
            rating: None,
            genres: Vec::new(),
            director: None,
            starring: None,
            series_tag: None,
            needs_tmdb_scan: true,
            is_watched: false,
            added_at: now,
        };
        skeletons.push(skeleton);
    }

    if !skeletons.is_empty() {
        let _ = crate::database::queries::save_media_items(&app, skeletons.clone());
        let _ = app.emit("flux-library-updated", ());
    }

    // --- STAGE 2: Slow Pass (Concurrent Incremental Enrichment) ---
    let show_cache: Arc<Mutex<std::collections::HashMap<String, MediaMetadata>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));

    // Re-calculate total_new for Stage 2 (actual enrichment count)
    let total_to_enrich = new_paths.len();
    let progress_counter = Arc::new(AtomicUsize::new(0));

    use futures::stream::{self, StreamExt};

    let mut stream = stream::iter(new_paths).map(|path| {
        let app_clone = app.clone();
        let cache_clone = Arc::clone(&show_cache);
        let counter_clone = Arc::clone(&progress_counter);

        async move {
            let current = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
            let _ = app_clone.emit("flux-scan-progress", (current, total_to_enrich));

            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();

                if metadata::is_video(&ext_lower) {
                    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
                    let (cleaned_title, _, extracted_series) = metadata::clean_media_title(file_stem);

                    let cached_show = if extracted_series.is_some() {
                        let cache = cache_clone.lock().await;
                        cache.get(&cleaned_title).cloned()
                    } else {
                        None
                    };

                    if let Some(meta) = metadata::process_video(&app_clone, &path, now, None, None, cached_show).await {
                        if extracted_series.is_some() {
                            let mut cache = cache_clone.lock().await;
                            if let std::collections::hash_map::Entry::Vacant(e) = cache.entry(cleaned_title) {
                                e.insert(meta.clone());
                            }
                        }

                        let _ = crate::database::queries::save_media_items(&app_clone, vec![meta.clone()]);
                        let _ = app_clone.emit("flux-item-enriched", meta.clone());
                        return Some(meta);
                    }
                } else if metadata::is_audio(&ext_lower) {
                    if let Some(meta) = metadata::process_audio(&app_clone, &path, now, None, None, None).await {
                        let _ = crate::database::queries::save_media_items(&app_clone, vec![meta.clone()]);
                        let _ = app_clone.emit("flux-item-enriched", meta.clone());
                        return Some(meta);
                    }
                }
            }
            None
        }
    }).buffer_unordered(10); // Buffer up to 10 HTTP requests and IO simultaneously!

    let mut enriched_results = Vec::new();
    while let Some(res) = stream.next().await {
        if let Some(meta) = res {
            enriched_results.push(meta);
        }
    }

    let _ = app.emit("flux-library-updated", ());
    enriched_results
}

pub async fn healing_sync<R: Runtime>(app: AppHandle<R>) -> usize {
    use futures::stream::{self, StreamExt};
    let show_cache: Arc<Mutex<std::collections::HashMap<String, MediaMetadata>>> =
        Arc::new(Mutex::new(std::collections::HashMap::new()));

    // 1. Fetch missing items securely
    let paths_to_heal = {
        let db_path = match crate::database::connection::get_db_path(&app) {
            Ok(p) => p,
            Err(_) => return 0,
        };
        let conn = match rusqlite::Connection::open(db_path) {
            Ok(c) => c,
            Err(_) => return 0,
        };

        let mut stmt = match conn.prepare(
            "SELECT path, added_at, title, poster_path FROM media WHERE needs_tmdb_scan = 1",
        ) {
            Ok(s) => s,
            Err(_) => return 0,
        };

        let mut paths = Vec::new();
        if let Ok(mut rows) = stmt.query([]) {
            while let Ok(Some(row)) = rows.next() {
                let path: String = row.get(0).unwrap_or_default();
                let added_at: u64 = row.get(1).unwrap_or(0);
                let title: Option<String> = row.get(2).ok();
                let poster: Option<String> = row.get(3).ok();
                paths.push((path, added_at, title, poster));
            }
        }
        paths
    };

    if paths_to_heal.is_empty() {
        return 0; // Everything is clean, no backlog.
    }

    let mut healed_count = 0;
    let total_to_heal = paths_to_heal.len();
    println!("[Flux Scanner] Healing Sync: Found {} items to heal.", total_to_heal);
    let progress_counter = Arc::new(AtomicUsize::new(0));

    let mut stream = stream::iter(paths_to_heal).map(|(path_str, added_at, ex_title, ex_poster)| {
        let app_clone = app.clone();
        let cache_clone = Arc::clone(&show_cache);
        let counter_clone = Arc::clone(&progress_counter);

        async move {
            let current = counter_clone.fetch_add(1, Ordering::SeqCst) + 1;
            let _ = app_clone.emit("flux-scan-progress", (current, total_to_heal));

            let path = std::path::PathBuf::from(&path_str);
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                let ext_lower = ext.to_lowercase();
                if metadata::is_video(&ext_lower) {
                    let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
                    let (cleaned_title, _, extracted_series) = metadata::clean_media_title(file_stem);

                    let cached_show = if extracted_series.is_some() {
                        let cache = cache_clone.lock().await;
                        cache.get(&cleaned_title).cloned()
                    } else {
                        None
                    };

                    if let Some(meta) = metadata::process_video(&app_clone, &path, added_at, ex_title, ex_poster, cached_show).await {
                        if extracted_series.is_some() {
                            let mut cache = cache_clone.lock().await;
                            if let std::collections::hash_map::Entry::Vacant(e) = cache.entry(cleaned_title) {
                                e.insert(meta.clone());
                            }
                        }

                        let _ = crate::database::queries::save_media_items(&app_clone, vec![meta.clone()]);
                        let _ = app_clone.emit("flux-item-enriched", meta.clone());
                        return 1;
                    }
                }
            }
            0
        }
    }).buffer_unordered(10);

    while let Some(res) = stream.next().await {
        healed_count += res;
    }

    if healed_count > 0 {
        println!("[Flux Scanner] Healing Sync complete. {} items healed.", healed_count);
        let _ = app.emit("flux-library-updated", ());
    }

    healed_count
}

#[cfg(test)]
mod concurrent_tests;
