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
        let genres_json = serde_json::to_string(&item.genres).unwrap_or_else(|_| "[]".to_string());
        
        tx.execute(
            "INSERT INTO media (
                path, title, year, artist, album, poster_path, backdrop_path, album_art_path, duration, media_type, added_at,
                synopsis, rating, genres, director, starring, series_tag, is_watched
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)
            ON CONFLICT(path) DO UPDATE SET
                title=excluded.title,
                year=excluded.year,
                artist=excluded.artist,
                album=excluded.album,
                poster_path=excluded.poster_path,
                backdrop_path=excluded.backdrop_path,
                album_art_path=excluded.album_art_path,
                duration=excluded.duration,
                media_type=excluded.media_type,
                synopsis=excluded.synopsis,
                rating=excluded.rating,
                genres=excluded.genres,
                director=excluded.director,
                starring=excluded.starring,
                series_tag=excluded.series_tag
            ",
            rusqlite::params![
                &item.path, &item.title, item.year, &item.artist, &item.album,
                &item.poster_path, &item.backdrop_path, &item.album_art_path,
                item.duration, &item.media_type, item.added_at,
                &item.synopsis, item.rating, &genres_json, &item.director, &item.starring, &item.series_tag, item.is_watched
            ],
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}
