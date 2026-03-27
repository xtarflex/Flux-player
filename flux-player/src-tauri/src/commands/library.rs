use crate::database;
use crate::scanner;
use crate::utils::os;
use std::io::Write;
use tauri::{AppHandle, Manager, Runtime};

#[tauri::command]
pub async fn start_library_scan<R: Runtime>(app: AppHandle<R>, dir: String) -> Result<(), String> {
    let items = scanner::scan_directory(app.clone(), dir).await;
    database::queries::save_media_items(&app, items)?;
    Ok(())
}

#[tauri::command]
pub fn get_file_oshash(path: String) -> Result<String, String> {
    os::compute_oshash(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn cache_tmdb_image<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    image_type: String, // "posters" or "backdrops" or "album-art"
) -> Result<String, String> {
    if url.is_empty() {
        return Err("EMPTY_URL".to_string());
    }

    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e: tauri::Error| e.to_string())?;
    let cache_dir = app_dir.join("cache").join("images").join(image_type);

    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    }

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let hash = format!("{:x}", hasher.finalize())[..16].to_string();
    let file_extension = url.split('.').last().unwrap_or("jpg");
    let file_name = format!("{}.{}", hash, file_extension);
    let target_path = cache_dir.join(&file_name);

    if !target_path.exists() {
        let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;
        let bytes = response.bytes().await.map_err(|e| e.to_string())?;
        let mut file = std::fs::File::create(&target_path).map_err(|e| e.to_string())?;
        file.write_all(&bytes).map_err(|e| e.to_string())?;
    }

    let encoded_path = target_path.to_string_lossy().replace("\\", "/");
    #[cfg(target_os = "windows")]
    {
        let escaped_path = encoded_path.replace(":", "%3A");
        Ok(format!("https://asset.localhost/{}", escaped_path))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(format!("https://asset.localhost{}", encoded_path))
    }
}
