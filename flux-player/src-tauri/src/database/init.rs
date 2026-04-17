use crate::database::connection::get_db_path;
use crate::database::schema::get_migrations;
use crate::utils::error::AppResult;
use rusqlite::Connection;
use tauri::{AppHandle, Runtime};

pub fn initialize_database<R: Runtime>(app: &AppHandle<R>) -> AppResult<()> {
    let db_path = get_db_path(app)?;

    // Ensure the data directory exists before opening the connection
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut conn = Connection::open(db_path)?;

    // Ensure migrations table exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS __flux_migrations (
            version INTEGER PRIMARY KEY,
            applied_at INTEGER NOT NULL
        )",
        [],
    )?;

    // Get current version
    let current_version: i32 = conn
        .query_row(
            "SELECT COALESCE(MAX(version), 0) FROM __flux_migrations",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let migrations = get_migrations();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let tx = conn.transaction()?;

    for migration in migrations {
        if migration.version as i32 > current_version {
            println!(
                "[Flux DB] Applying migration {}: {}",
                migration.version, migration.description
            );
            // Execute the SQL. Note: tauri_plugin_sql uses &str, rusqlite execute also takes &str.
            // Some migrations might contain multiple statements which rusqlite's execute() won't handle
            // unless we use execute_batch().
            tx.execute_batch(migration.sql)?;

            tx.execute(
                "INSERT INTO __flux_migrations (version, applied_at) VALUES (?1, ?2)",
                (migration.version as i32, now),
            )?;
        }
    }

    tx.commit()?;
    println!("[Flux DB] Database schema is up to date.");

    // Ensure default settings exist
    ensure_default_settings(&conn)?;

    Ok(())
}

/// Helper for unit tests to bootstrap a database connection.
#[cfg(test)]
pub fn initialize_database_for_tests(conn: &rusqlite::Connection) {
    // Ensure migrations table exists
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS __flux_migrations (
            version INTEGER PRIMARY KEY,
            applied_at INTEGER NOT NULL
        )",
        [],
    );

    let migrations = crate::database::schema::get_migrations();
    for migration in migrations {
        let _ = conn.execute_batch(migration.sql);
    }
}

fn ensure_default_settings(conn: &Connection) -> AppResult<()> {
    let defaults = [
        ("autoIndexing", "true"),
        ("scanFrequency", "30"),
        ("hwAcceleration", "true"),
        ("ffmpegThreading", "Auto"),
        ("watchedThreshold", "90"),
        ("videoAudioTransition", "true"),
        ("theme", "Cyber Dark"),
        ("language", "System Default"),
    ];

    for (key, value) in defaults {
        conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2) ON CONFLICT(key) DO NOTHING",
            (key, value),
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_migrations_full_chain() {
        // Use an in-memory database to test the migrations from scratch
        let conn = Connection::open_in_memory().expect("Should be able to open in-memory DB");

        // 1. Setup the migrations table (normally handled by initialize_database)
        conn.execute(
            "CREATE TABLE __flux_migrations (
                version INTEGER PRIMARY KEY,
                applied_at INTEGER NOT NULL
            )",
            [],
        )
        .unwrap();

        // 2. Fetch and apply all migrations
        let migrations = get_migrations();
        let now = 1599670551; // Static timestamp for testing

        for migration in &migrations {
            // Apply migration SQL
            conn.execute_batch(migration.sql).expect(&format!(
                "Migration {} ({}) failed to apply",
                migration.version, migration.description
            ));

            // Record migration
            conn.execute(
                "INSERT INTO __flux_migrations (version, applied_at) VALUES (?1, ?2)",
                (migration.version as i32, now),
            )
            .unwrap();
        }

        // 3. Verify the final schema state
        // Check core tables
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        assert!(tables.contains(&"media".to_string()));
        assert!(tables.contains(&"playlists".to_string()));
        assert!(tables.contains(&"settings".to_string()));

        // Check specific columns from later migrations (v5 to v12)
        let stmt = conn.prepare("SELECT * FROM media LIMIT 0").unwrap();
        let column_names: Vec<String> = stmt
            .column_names()
            .into_iter()
            .map(|n| n.to_string())
            .collect();

        assert!(
            column_names.contains(&"rating".to_string()),
            "Migration v6 (rating) missing"
        );
        assert!(
            column_names.contains(&"genres".to_string()),
            "Migration v7 (genres) missing"
        );
        assert!(
            column_names.contains(&"series_tag".to_string()),
            "Migration v10 (series_tag) missing"
        );
        assert!(
            column_names.contains(&"is_watched".to_string()),
            "Migration v10 (is_watched) missing"
        );
        assert!(
            column_names.contains(&"last_position".to_string()),
            "Migration v11 (playback) missing"
        );
        assert!(
            column_names.contains(&"is_favorite".to_string()),
            "Migration v12 (favorites) missing"
        );

        println!(
            "[Flux DB Test] All {} migrations verified successfully.",
            migrations.len()
        );
    }
}
