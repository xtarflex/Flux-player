use crate::database;
use crate::database::models::LibraryItem;
use crate::scanner;
use crate::utils::error::AppResult;
use crate::utils::os;
use std::io::Write;
use tauri::{AppHandle, Manager, Runtime};

#[tauri::command]
pub async fn start_library_scan<R: Runtime>(app: AppHandle<R>, dir: String) -> AppResult<()> {
    let items = scanner::scan_directory(app.clone(), dir).await;
    database::queries::save_media_items(&app, items)?;
    Ok(())
}

#[tauri::command]
pub async fn heal_library<R: Runtime>(app: AppHandle<R>) -> AppResult<usize> {
    Ok(scanner::healing_sync(app).await)
}

#[tauri::command]
pub fn get_file_oshash(path: String) -> AppResult<String> {
    Ok(os::compute_oshash(&path)?)
}

#[tauri::command]
pub async fn cache_tmdb_image<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    image_type: String, // "posters" or "backdrops" or "album-art"
) -> AppResult<String> {
    if url.is_empty() {
        return Err(crate::utils::error::AppError::InvalidInput(
            "EMPTY_URL".into(),
        ));
    }

    let app_dir = app.path().app_data_dir()?;
    let cache_dir = app_dir.join("cache").join("images").join(image_type);

    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir)?;
    }

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(url.as_bytes());
    let hash = format!("{:x}", hasher.finalize())[..16].to_string();
    let file_extension = url.split('.').next_back().unwrap_or("jpg");
    let file_name = format!("{}.{}", hash, file_extension);
    let target_path = cache_dir.join(&file_name);

    if !target_path.exists() {
        let response = reqwest::get(&url).await?;
        let bytes = response.bytes().await?;
        let mut file = std::fs::File::create(&target_path)?;
        file.write_all(&bytes)?;
    }

    Ok(target_path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn refresh_media_metadata<R: Runtime>(app: AppHandle<R>, path: String) -> AppResult<()> {
    let p = std::path::Path::new(&path);
    let _name = p
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| crate::utils::error::AppError::InvalidInput("INVALID_PATH".into()))?;
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
        let db_path = crate::database::connection::get_db_path(&app)?;
        let conn = rusqlite::Connection::open(db_path)?;

        let mut stmt = conn.prepare("SELECT title, artist, album FROM media WHERE path = ?1")?;

        let mut rows = stmt.query([&path])?;

        if let Some(row) = rows.next()? {
            let t: Option<String> = row.get(0).ok();
            let a: Option<String> = row.get(1).ok();
            let al: Option<String> = row.get(2).ok();
            (t, a, al)
        } else {
            (None, None, None)
        }
    };

    let result = if scanner::metadata::is_video(&ext) {
        scanner::metadata::process_video(&app, p, added_at, ex_title, None).await
    } else if scanner::metadata::is_audio(&ext) {
        scanner::metadata::process_audio(&app, p, added_at, ex_title, ex_artist, ex_album).await
    } else {
        None
    };

    if let Some(meta) = result {
        database::queries::save_media_items(&app, vec![meta])?;
        Ok(())
    } else {
        Err(crate::utils::error::AppError::Internal(
            "FAILED_TO_PROCESS".into(),
        ))
    }
}

#[tauri::command]
pub async fn update_media_field<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    field: String,
    value: String,
) -> AppResult<()> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    let query = match field.as_str() {
        "title" => "UPDATE media SET title = ?1 WHERE path = ?2",
        "artist" => "UPDATE media SET artist = ?1 WHERE path = ?2",
        "album" => "UPDATE media SET album = ?1 WHERE path = ?2",
        _ => {
            return Err(crate::utils::error::AppError::InvalidInput(
                "INVALID_FIELD".into(),
            ))
        }
    };

    conn.execute(query, (&value, &path))?;

    Ok(())
}

/// Saves the current playback position (seconds) for a media file.
/// Automatically marks the file as watched when reaching the user-defined threshold (default 90%).
#[tauri::command]
pub fn save_playback_progress<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    seconds: i64,
    duration: i64,
) -> AppResult<()> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    // 1. Resolve watched threshold (Default 90 if not set)
    let threshold_val: i64 = conn
        .query_row(
            "SELECT value FROM settings WHERE key = 'watchedThreshold'",
            [],
            |row| {
                row.get(0).and_then(|v: String| {
                    v.parse::<i64>().map_err(|_| rusqlite::Error::InvalidQuery)
                })
            },
        )
        .unwrap_or(90);

    let threshold_factor = threshold_val as f64 / 100.0;

    // 2. Fetch existing status and position to handle "Threshold Crossing"
    let (prev_is_watched, prev_position): (i64, i64) = conn
        .query_row(
            "SELECT is_watched, last_position FROM media WHERE path = ?1",
            [&path],
            |row| Ok((row.get(0).unwrap_or(0), row.get(1).unwrap_or(0))),
        )
        .unwrap_or((0, 0));

    let threshold_seconds = (duration as f64 * threshold_factor) as i64;

    // 3. Calculate New Watched Status (Threshold Crossing Logic)
    let is_watched = if prev_is_watched == 1
        || (duration > 0 && prev_position < threshold_seconds && seconds >= threshold_seconds)
    {
        1
    } else {
        prev_is_watched
    };

    // 4. Smart Progress: If it's finished (reset to 0 by frontend) or crosses threshold,
    // handle the last_position accordingly.
    let last_position = if seconds == 0 && is_watched == 1 {
        0
    } else {
        seconds
    };

    conn.execute(
        "UPDATE media SET last_position = ?1, is_watched = ?2 WHERE path = ?3",
        rusqlite::params![last_position, is_watched as i64, &path],
    )?;

    Ok(())
}

/// Toggles the watched status of a media item manually (e.g. via 'W' key).
#[tauri::command]
pub async fn toggle_media_watched_status<R: Runtime>(
    app: AppHandle<R>,
    path: String,
) -> AppResult<bool> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    let current_status: i64 = conn
        .query_row(
            "SELECT is_watched FROM media WHERE path = ?1",
            [&path],
            |row| row.get(0),
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                crate::utils::error::AppError::NotFound("MEDIA_NOT_FOUND".into())
            }
            _ => crate::utils::error::AppError::Database(e),
        })?;

    let new_status = if current_status == 1 { 0 } else { 1 };

    conn.execute(
        "UPDATE media SET is_watched = ?1 WHERE path = ?2",
        rusqlite::params![new_status, &path],
    )?;

    Ok(new_status == 1)
}

/// Fetches the stored playback position (seconds) for a media file.
#[tauri::command]
pub fn get_playback_progress<R: Runtime>(app: AppHandle<R>, path: String) -> AppResult<i64> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    let result: rusqlite::Result<i64> = conn.query_row(
        "SELECT last_position FROM media WHERE path = ?1",
        [&path],
        |row| row.get(0),
    );

    match result {
        Ok(pos) => Ok(pos),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(0),
        Err(e) => Err(crate::utils::error::AppError::Database(e)),
    }
}

/// Fetches all media items from the database.
#[tauri::command]
pub async fn get_all_media<R: Runtime>(app: AppHandle<R>) -> AppResult<Vec<LibraryItem>> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    let mut stmt = conn.prepare(
        "SELECT path, title, year, artist, album, poster_path, backdrop_path, album_art_path, 
                duration, media_type, last_played, added_at, synopsis, rating, genres, 
                director, starring, series_tag, is_watched, last_position, is_favorite, needs_tmdb_scan
         FROM media ORDER BY added_at DESC"
    )?;

    let media_iter = stmt.query_map([], |row| {
        let genres_raw: Option<String> = row.get(14)?;
        let genres: Vec<String> = genres_raw
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default();

        Ok(LibraryItem {
            path: row.get(0)?,
            title: row.get(1)?,
            year: row.get(2)?,
            artist: row.get(3)?,
            album: row.get(4)?,
            poster_path: row.get(5)?,
            backdrop_path: row.get(6)?,
            album_art_path: row.get(7)?,
            duration: row.get(8)?,
            media_type: row.get(9)?,
            last_played: row.get(10)?,
            added_at: row.get(11)?,
            synopsis: row.get(12)?,
            rating: row.get(13)?,
            genres,
            director: row.get(15)?,
            starring: row.get(16)?,
            series_tag: row.get(17)?,
            is_watched: row.get::<_, i64>(18)? == 1,
            last_position: row.get(19)?,
            is_favorite: row.get::<_, i64>(20)? == 1,
            needs_tmdb_scan: row.get::<_, i64>(21)? == 1,
        })
    })?;

    let mut items = Vec::new();
    for item in media_iter {
        items.push(item?);
    }

    Ok(items)
}

/// Toggles the favorite status of a media item.
#[tauri::command]
pub async fn toggle_favorite_status<R: Runtime>(
    app: AppHandle<R>,
    path: String,
) -> AppResult<bool> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    let current_status: i64 = conn
        .query_row(
            "SELECT is_favorite FROM media WHERE path = ?1",
            [&path],
            |row| row.get(0),
        )
        .map_err(|e| match e {
            rusqlite::Error::QueryReturnedNoRows => {
                crate::utils::error::AppError::NotFound("MEDIA_NOT_FOUND".into())
            }
            _ => crate::utils::error::AppError::Database(e),
        })?;

    let new_status = if current_status == 1 { 0 } else { 1 };

    conn.execute(
        "UPDATE media SET is_favorite = ?1 WHERE path = ?2",
        rusqlite::params![new_status, &path],
    )?;

    Ok(new_status == 1)
}
