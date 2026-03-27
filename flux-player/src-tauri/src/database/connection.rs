use std::path::PathBuf;
use tauri::{AppHandle, Manager, Runtime};

pub fn get_db_path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf, String> {
    let app_dir = app
        .path()
        .app_data_dir()
        .map_err(|e: tauri::Error| e.to_string())?;
    Ok(app_dir.join("flux.db"))
}
