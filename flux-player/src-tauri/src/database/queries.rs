use crate::database::connection;
use crate::scanner::MediaMetadata;
use tauri::{AppHandle, Runtime};

pub fn save_media_items<R: Runtime>(
    app: &AppHandle<R>,
    items: Vec<MediaMetadata>,
) -> Result<(), String> {
    let db_path = connection::get_db_path(app)?;

    let mut conn = rusqlite::Connection::open(db_path).map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    for item in items {
        tx.execute(
            "INSERT OR REPLACE INTO media (
                path, title, year, artist, album, poster_path, backdrop_path, album_art_path, duration, media_type, added_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
            (
                &item.path, &item.title, item.year, &item.artist, &item.album,
                &item.poster_path, &item.backdrop_path, &item.album_art_path,
                item.duration, &item.media_type, item.added_at
            ),
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}
