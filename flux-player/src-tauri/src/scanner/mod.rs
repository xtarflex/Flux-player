pub mod crawler;
pub mod metadata;
pub mod tmdb;

pub use metadata::MediaMetadata;
use std::time::{SystemTime, UNIX_EPOCH};
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
        // We use a prefix or float calculation if we want a single bar,
        // but for now we'll just emit progress normally.
        let _ = app.emit("flux-scan-progress", (index + 1, skeleton_total));

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_lowercase();
            let is_v = metadata::is_video(&ext_lower);
            let is_a = metadata::is_audio(&ext_lower);

            if is_v || is_a {
                let file_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                let (cleaned_title, year, series) = metadata::clean_media_title(file_stem);

                skeletons.push(MediaMetadata {
                    path: path.to_string_lossy().to_string(),
                    title: cleaned_title,
                    year,
                    artist: None,
                    album: None,
                    poster_path: None,
                    backdrop_path: None,
                    album_art_path: None,
                    duration: None,
                    media_type: if is_v {
                        "video".to_string()
                    } else {
                        "audio".to_string()
                    },
                    synopsis: None,
                    rating: None,
                    genres: Vec::new(),
                    director: None,
                    starring: None,
                    series_tag: series,
                    is_watched: false,
                    added_at: now,
                    needs_tmdb_scan: is_v, // Only videos strictly need TMDB enrichment by default
                });
            }
        }
    }

    if !skeletons.is_empty() {
        let _ = crate::database::queries::save_media_items(&app, skeletons.clone());
        let _ = app.emit("flux-library-updated", ());
    }

    // --- STAGE 2: Slow Pass (Incremental Enrichment) ---
    let mut enriched_results = Vec::new();
    let mut show_cache: std::collections::HashMap<String, MediaMetadata> =
        std::collections::HashMap::new();

    // Re-calculate total_new for Stage 2 (actual enrichment count)
    let total_to_enrich = new_paths.len();

    for (index, path) in new_paths.into_iter().enumerate() {
        // Emit progress to frontend for enrichment stage
        let _ = app.emit("flux-scan-progress", (index + 1, total_to_enrich));

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_lowercase();
            if metadata::is_video(&ext_lower) {
                let file_stem = path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or_default();
                let (cleaned_title, _, extracted_series) = metadata::clean_media_title(file_stem);

                let cached_show = if extracted_series.is_some() {
                    show_cache.get(&cleaned_title).cloned()
                } else {
                    None
                };

                if let Some(meta) =
                    metadata::process_video(&app, &path, now, None, None, cached_show).await
                {
                    if extracted_series.is_some() && !show_cache.contains_key(&cleaned_title) {
                        show_cache.insert(cleaned_title, meta.clone());
                    }

                    // Save individual item to DB
                    let _ = crate::database::queries::save_media_items(&app, vec![meta.clone()]);
                    let _ = app.emit("flux-item-enriched", meta.clone());
                    enriched_results.push(meta);
                }
            } else if metadata::is_audio(&ext_lower) {
                if let Some(meta) =
                    metadata::process_audio(&app, &path, now, None, None, None).await
                {
                    let _ = crate::database::queries::save_media_items(&app, vec![meta.clone()]);
                    let _ = app.emit("flux-item-enriched", meta.clone());
                    enriched_results.push(meta);
                }
            }
        }
    }

    enriched_results
}

pub async fn healing_sync<R: Runtime>(app: AppHandle<R>) -> usize {
    let db_path = match crate::database::connection::get_db_path(&app) {
        Ok(p) => p,
        Err(_) => return 0,
    };

    let paths_to_heal = {
        let conn = match rusqlite::Connection::open(&db_path) {
            Ok(c) => c,
            Err(_) => return 0,
        };
        let mut stmt = match conn.prepare("SELECT path, added_at, title, poster_path FROM media WHERE needs_tmdb_scan = 1 AND media_type = 'video'") {
            Ok(s) => s,
            Err(_) => return 0,
        };

        let mut rows = match stmt.query([]) {
            Ok(r) => r,
            Err(_) => return 0,
        };

        let mut paths = Vec::new();
        while let Ok(Some(row)) = rows.next() {
            let p: String = row.get(0).unwrap_or_default();
            let a: i64 = row.get(1).unwrap_or_default();
            let t: Option<String> = row.get(2).ok();
            let po: Option<String> = row.get(3).ok();
            paths.push((p, a as u64, t, po));
        }
        paths
    };

    if paths_to_heal.is_empty() {
        return 0;
    }

    let total_to_heal = paths_to_heal.len();
    if total_to_heal == 0 {
        return 0;
    }

    let mut show_cache: std::collections::HashMap<String, MediaMetadata> =
        std::collections::HashMap::new();
    let mut results = Vec::new();

    for (index, (path_str, added_at, ex_title, ex_poster)) in paths_to_heal.into_iter().enumerate()
    {
        let _ = app.emit("flux-heal-progress", (index + 1, total_to_heal));

        let path = std::path::Path::new(&path_str);
        if !path.exists() {
            continue;
        }

        let file_stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or_default();
        let (cleaned_title, _, extracted_series) = metadata::clean_media_title(file_stem);

        let cached_show = if extracted_series.is_some() {
            show_cache.get(&cleaned_title).cloned()
        } else {
            None
        };

        if let Some(meta) =
            metadata::process_video(&app, path, added_at, ex_title, ex_poster, cached_show).await
        {
            let quota_exceeded = meta.needs_tmdb_scan;
            if !quota_exceeded {
                // Update show_cache for siblings in this batch
                if extracted_series.is_some() {
                    show_cache.insert(cleaned_title.clone(), meta.clone());
                }
                results.push(meta);
            } else {
                // Network error / Out of quota: Stop healing this batch
                break;
            }
        }
    }

    let healed_count = results.len();
    if healed_count > 0 {
        let _ = crate::database::queries::save_media_items(&app, results);
        let _ = app.emit("flux-library-updated", ());
    }

    healed_count
}
