use std::path::PathBuf;
use tauri::{AppHandle, Runtime, Manager};
use crate::utils::error::AppResult;

pub fn get_db_path<R: Runtime>(app: &AppHandle<R>) -> AppResult<PathBuf> {
    let app_dir = app
        .path()
        .app_data_dir()?;
    Ok(app_dir.join("flux.db"))
}
