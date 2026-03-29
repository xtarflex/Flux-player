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

    Ok(target_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn refresh_media_metadata<R: Runtime>(
    app: AppHandle<R>,
    path: String,
) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    let _name = p
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or("INVALID_PATH")?;
    let ext = p
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    let added_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Fetch existing title, artist, album if they were manually edited
    // Use a strict block to drop the non-Send SQLite connection before the .await
    let (ex_title, ex_artist, ex_album) = {
        let db_path = crate::database::connection::get_db_path(&app).map_err(|e| e.to_string())?;
        let conn = rusqlite::Connection::open(db_path).map_err(|e| e.to_string())?;
        
        let mut stmt = conn
            .prepare("SELECT title, artist, album FROM media WHERE path = ?1")
            .map_err(|e| e.to_string())?;
            
        let mut rows = stmt.query([&path]).map_err(|e| e.to_string())?;
        
        if let Some(row) = rows.next().map_err(|e| e.to_string())? {
            let t: Option<String> = row.get(0).ok();
            let a: Option<String> = row.get(1).ok();
            let al: Option<String> = row.get(2).ok();
            (t, a, al)
        } else {
            (None, None, None)
        }
    };

    let result = if scanner::metadata::is_video(&ext) {
        scanner::metadata::process_video(&app, p, added_at, ex_title).await
    } else if scanner::metadata::is_audio(&ext) {
        scanner::metadata::process_audio(&app, p, added_at, ex_title, ex_artist, ex_album).await
    } else {
        None
    };

    if let Some(meta) = result {
        database::queries::save_media_items(&app, vec![meta])?;
        Ok(())
    } else {
        Err("FAILED_TO_PROCESS".to_string())
    }
}

#[tauri::command]
pub async fn update_media_field<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    field: String,
    value: String,
) -> Result<(), String> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path).map_err(|e| e.to_string())?;

    let query = match field.as_str() {
        "title" => "UPDATE media SET title = ?1 WHERE path = ?2",
        "artist" => "UPDATE media SET artist = ?1 WHERE path = ?2",
        "album" => "UPDATE media SET album = ?1 WHERE path = ?2",
        _ => return Err("INVALID_FIELD".to_string()),
    };

    conn.execute(query, (&value, &path))
        .map_err(|e| e.to_string())?;

    Ok(())
}

/// Saves the current playback position (seconds) for a media file.
/// Automatically marks the file as watched when >= 90% of duration is complete.
///
/// @param path - Absolute file path (PRIMARY KEY in the media table).
/// @param seconds - Current playback position in seconds.
/// @param duration - Total duration in seconds (used for watched threshold).
#[tauri::command]
pub fn save_playback_progress<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    seconds: i64,
    duration: i64,
) -> Result<(), String> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path).map_err(|e| e.to_string())?;

    // Calculate watched status: 90% completion rule
    let is_watched = duration > 0 && (seconds as f64 / duration as f64) >= 0.9;

    conn.execute(
        "UPDATE media SET last_position = ?1, is_watched = ?2 WHERE path = ?3",
        rusqlite::params![seconds, is_watched as i64, &path],
    )
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Fetches the stored playback position (seconds) for a media file.
///
/// @param path - Absolute file path (PRIMARY KEY in the media table).
/// @returns The last_position in seconds, or 0 if not found.
#[tauri::command]
pub fn get_playback_progress<R: Runtime>(
    app: AppHandle<R>,
    path: String,
) -> Result<i64, String> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path).map_err(|e| e.to_string())?;

    let result: rusqlite::Result<i64> = conn.query_row(
        "SELECT last_position FROM media WHERE path = ?1",
        [&path],
        |row| row.get(0),
    );

    match result {
        Ok(pos) => Ok(pos),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(e) => Err(e.to_string()),
    }
}
