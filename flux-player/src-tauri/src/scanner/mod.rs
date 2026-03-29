pub mod crawler;
pub mod metadata;
pub mod tmdb;

pub use metadata::MediaMetadata;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Runtime};

pub async fn scan_directory<R: Runtime>(app: AppHandle<R>, dir_path: String) -> Vec<MediaMetadata> {
    let mut results = Vec::new();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let paths = crawler::walk_directory(&dir_path);
    for path in paths {
        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            let ext_lower = ext.to_lowercase();
            if metadata::is_video(&ext_lower) {
                if let Some(meta) = metadata::process_video(&app, &path, now, None).await {
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
