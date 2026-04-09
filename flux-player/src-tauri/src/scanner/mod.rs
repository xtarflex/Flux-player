pub mod crawler;
pub mod metadata;
pub mod tmdb;

pub use metadata::MediaMetadata;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Runtime, Emitter};

use std::collections::HashSet;

pub async fn scan_directory<R: Runtime>(app: AppHandle<R>, dir_path: String) -> Vec<MediaMetadata> {
    let mut results = Vec::new();
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

    let mut show_cache: std::collections::HashMap<String, MediaMetadata> = std::collections::HashMap::new();

    let paths = crawler::walk_directory(&dir_path);
    let total_found = paths.len();
    
    for (index, path) in paths.into_iter().enumerate() {
        // Emit progress to frontend
        let _ = app.emit("flux-scan-progress", (index + 1, total_found));
        
        let path_str = path.to_string_lossy().to_string();
        if existing_paths.contains(&path_str) {
            continue; // O(1) skip to save API quota
        }

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_lowercase();
            if metadata::is_video(&ext_lower) {
                let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
                let (cleaned_title, _, extracted_series) = metadata::clean_media_title(file_stem);
                
                let cached_show = if extracted_series.is_some() {
                    show_cache.get(&cleaned_title).cloned()
                } else {
                    None
                };

                if let Some(meta) = metadata::process_video(&app, &path, now, None, cached_show).await {
                    if extracted_series.is_some() && !show_cache.contains_key(&cleaned_title) {
                        show_cache.insert(cleaned_title, meta.clone());
                    }
                    results.push(meta);
                }
            } else if metadata::is_audio(&ext_lower) {
                if let Some(meta) = metadata::process_audio(&app, &path, now, None, None, None).await {
                    results.push(meta);
                }
            }
        }
    }
    results
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
        let mut stmt = match conn.prepare("SELECT path, added_at, title FROM media WHERE needs_tmdb_scan = 1 AND media_type = 'video'") {
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
            paths.push((p, a as u64, t));
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
    
    let mut show_cache: std::collections::HashMap<String, MediaMetadata> = std::collections::HashMap::new();
    let mut results = Vec::new();
    
    for (index, (path_str, added_at, ex_title)) in paths_to_heal.into_iter().enumerate() {
        let _ = app.emit("flux-heal-progress", (index + 1, total_to_heal));
        
        let path = std::path::Path::new(&path_str);
        if !path.exists() {
            continue;
        }
        
        let file_stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or_default();
        let (cleaned_title, _, extracted_series) = metadata::clean_media_title(file_stem);
        
        let cached_show = if extracted_series.is_some() {
            show_cache.get(&cleaned_title).cloned()
        } else {
            None
        };
        
        if let Some(meta) = metadata::process_video(&app, path, added_at, ex_title, cached_show).await {
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
