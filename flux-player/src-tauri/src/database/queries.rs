use crate::database::connection;
use crate::scanner::MediaMetadata;
use crate::utils::error::AppResult;
use tauri::{AppHandle, Runtime};

pub fn clean_stale_media<R: Runtime>(app: &AppHandle<R>, dir_path: &str) -> AppResult<usize> {
    let db_path = connection::get_db_path(app)?;
    let mut conn = rusqlite::Connection::open(db_path)?;

    // Fetch all paths that start with the scanned directory
    // Use proper path separator handling to avoid matching siblings if they have the same prefix
    let mut dir_str = dir_path.to_string();
    if !dir_str.ends_with(std::path::MAIN_SEPARATOR) {
        dir_str.push(std::path::MAIN_SEPARATOR);
    }
    let like_pattern = format!("{}%", dir_str);

    let mut paths_to_check = Vec::new();
    {
        let mut stmt = conn.prepare("SELECT path FROM media WHERE path LIKE ?1")?;
        let mut rows = stmt.query([&like_pattern])?;
        while let Ok(Some(row)) = rows.next() {
            if let Ok(path) = row.get::<_, String>(0) {
                paths_to_check.push(path);
            }
        }
    }

    let mut stale_paths = Vec::new();
    for path in paths_to_check {
        if !std::path::Path::new(&path).exists() {
            stale_paths.push(path);
        }
    }

    let mut deleted_count = 0;
    if !stale_paths.is_empty() {
        let tx = conn.transaction()?;
        for path in stale_paths {
            tx.execute("DELETE FROM media WHERE path = ?1", rusqlite::params![path])?;
            deleted_count += 1;
        }
        tx.commit()?;
    }

    Ok(deleted_count)
}

pub fn save_media_items<R: Runtime>(
    app: &AppHandle<R>,
    items: Vec<MediaMetadata>,
) -> AppResult<()> {
    let db_path = connection::get_db_path(app)?;

    let mut conn = rusqlite::Connection::open(db_path)?;
    let tx = conn.transaction()?;

    for item in items {
        let genres_json = serde_json::to_string(&item.genres).unwrap_or_else(|_| "[]".to_string());

        tx.execute(
            "INSERT INTO media (
                path, title, year, artist, album, poster_path, backdrop_path, album_art_path, duration, media_type, added_at,
                synopsis, rating, genres, director, starring, series_tag, is_watched, needs_tmdb_scan
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)
            ON CONFLICT(path) DO UPDATE SET
                title=excluded.title,
                year=COALESCE(excluded.year, year),
                artist=COALESCE(excluded.artist, artist),
                album=COALESCE(excluded.album, album),
                poster_path=COALESCE(excluded.poster_path, poster_path),
                backdrop_path=COALESCE(excluded.backdrop_path, backdrop_path),
                album_art_path=COALESCE(excluded.album_art_path, album_art_path),
                duration=COALESCE(excluded.duration, duration),
                media_type=excluded.media_type,
                synopsis=COALESCE(excluded.synopsis, synopsis),
                rating=COALESCE(excluded.rating, rating),
                genres=COALESCE(excluded.genres, genres),
                director=COALESCE(excluded.director, director),
                starring=COALESCE(excluded.starring, starring),
                series_tag=COALESCE(excluded.series_tag, series_tag),
                needs_tmdb_scan=excluded.needs_tmdb_scan
            ",
            rusqlite::params![
                &item.path, &item.title, item.year, &item.artist, &item.album,
                &item.poster_path, &item.backdrop_path, &item.album_art_path,
                item.duration, &item.media_type, item.added_at,
                &item.synopsis, item.rating, &genres_json, &item.director, &item.starring, &item.series_tag, item.is_watched, item.needs_tmdb_scan
            ],
        )?;
    }

    tx.commit()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::database::init::initialize_database_for_tests;
    use crate::scanner::MediaMetadata;
    use tempfile::tempdir;

    #[test]
    fn test_clean_stale_media() {
        let dir = tempdir().expect("Failed to create temp dir");
        let db_path = dir.path().join("test_flux.db");
        let mut conn = rusqlite::Connection::open(&db_path).unwrap();
        initialize_database_for_tests(&conn);

        // 1. Create a real file in our temp dir
        let real_file_path = dir.path().join("real_video.mkv");
        std::fs::write(&real_file_path, "dummy data").unwrap();
        let real_path_str = real_file_path.to_string_lossy().to_string();

        // 2. Create a fake path that simulates a deleted file
        let fake_file_path = dir.path().join("deleted_video.mkv");
        let fake_path_str = fake_file_path.to_string_lossy().to_string();

        // 3. Insert both into the database
        conn.execute(
            "INSERT INTO media (path, title, media_type, added_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![&real_path_str, "Real Video", "video", 12345],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO media (path, title, media_type, added_at) VALUES (?1, ?2, ?3, ?4)",
            rusqlite::params![&fake_path_str, "Fake Video", "video", 12345],
        )
        .unwrap();

        // 4. Run cleanup logic directly (since we cant mock AppHandle easily, we replicate the logic)
        let dir_path_str = dir.path().to_string_lossy().to_string();
        let mut dir_str = dir_path_str.clone();
        if !dir_str.ends_with(std::path::MAIN_SEPARATOR) {
            dir_str.push(std::path::MAIN_SEPARATOR);
        }
        let like_pattern = format!("{}%", dir_str);

        let mut paths_to_check = Vec::new();
        {
            let mut stmt = conn
                .prepare("SELECT path FROM media WHERE path LIKE ?1")
                .unwrap();
            let mut rows = stmt.query([&like_pattern]).unwrap();
            while let Ok(Some(row)) = rows.next() {
                if let Ok(path) = row.get::<_, String>(0) {
                    paths_to_check.push(path);
                }
            }
        }

        let mut stale_paths = Vec::new();
        for path in paths_to_check {
            if !std::path::Path::new(&path).exists() {
                stale_paths.push(path);
            }
        }

        if !stale_paths.is_empty() {
            let tx = conn.transaction().unwrap();
            for path in stale_paths {
                tx.execute("DELETE FROM media WHERE path = ?1", rusqlite::params![path])
                    .unwrap();
            }
            tx.commit().unwrap();
        }

        // 5. Verify results
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM media", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1, "Should only have 1 item left");

        let remaining_path: String = conn
            .query_row("SELECT path FROM media", [], |row| row.get(0))
            .unwrap();
        assert_eq!(
            remaining_path, real_path_str,
            "The remaining item should be the real file"
        );
    }

    #[test]
    fn test_do_no_harm_metadata_protection() {
        let dir = tempdir().expect("Failed to create temp dir");
        let db_path = dir.path().join("test_flux.db");
        let conn = rusqlite::Connection::open(&db_path).unwrap();
        initialize_database_for_tests(&conn);

        // 1. Initial manual mock - "Partially Enriched"
        conn.execute(
            "INSERT INTO media (path, title, synopsis, poster_path, media_type, added_at, needs_tmdb_scan) 
             VALUES ('test.mkv', 'Test Title', 'Original Synopsis', 'original.jpg', 'video', 12345, 1)",
            []
        ).unwrap();

        // 2. Simulate a scan that returns NULL/empty for synopses (e.g. failed enrichment)
        let meta = MediaMetadata {
            path: "test.mkv".to_string(),
            title: "Updated Title".to_string(), // Titles ALWAYS update
            synopsis: None, // This is the core test: if it's null in result, keeps existing
            poster_path: None, // Same for posters
            backdrop_path: None,
            album: Some(String::new()),
            artist: Some(String::new()),
            album_art_path: Some(String::new()),
            year: None,
            duration: Some(120),
            media_type: "video".to_string(),
            added_at: 12345,
            rating: None,
            genres: vec![],
            director: None,
            starring: None,
            series_tag: None,
            is_watched: false,
            needs_tmdb_scan: false,
        };

        // We can't easily mock AppHandle in a pure unit test without heavy crates,
        // so we test the SQL logic directly here to verify the CONFLICT logic.
        let genres_json = "[]";
        conn.execute(
            "INSERT INTO media (
                path, title, year, artist, album, poster_path, backdrop_path, album_art_path, duration, media_type, added_at,
                synopsis, rating, genres, director, starring, series_tag, is_watched, needs_tmdb_scan
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)
            ON CONFLICT(path) DO UPDATE SET
                title=excluded.title,
                year=COALESCE(excluded.year, year),
                poster_path=COALESCE(excluded.poster_path, poster_path),
                synopsis=COALESCE(excluded.synopsis, synopsis),
                needs_tmdb_scan=excluded.needs_tmdb_scan
            ",
            rusqlite::params![
                &meta.path, &meta.title, meta.year, &meta.artist, &meta.album,
                &meta.poster_path, &meta.backdrop_path, &meta.album_art_path,
                meta.duration, &meta.media_type, meta.added_at,
                &meta.synopsis, meta.rating, &genres_json, &meta.director, &meta.starring, &meta.series_tag, meta.is_watched, meta.needs_tmdb_scan
            ],
        ).unwrap();

        // 3. Verify results
        let mut stmt = conn
            .prepare("SELECT title, synopsis, poster_path FROM media WHERE path = 'test.mkv'")
            .unwrap();
        let (title, synopsis, poster): (String, String, String) = stmt
            .query_row([], |row| {
                Ok((
                    row.get(0).unwrap(),
                    row.get(1).unwrap(),
                    row.get(2).unwrap(),
                ))
            })
            .unwrap();

        // Title should be updated
        assert_eq!(title, "Updated Title");
        // Synopsis should NOT be wiped with NULL
        assert_eq!(synopsis, "Original Synopsis");
        // Poster should NOT be wiped with NULL
        assert_eq!(poster, "original.jpg");
    }
}
