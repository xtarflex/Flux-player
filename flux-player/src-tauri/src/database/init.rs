use rusqlite::Connection;
use tauri::{AppHandle, Runtime};
use crate::database::connection::get_db_path;
use crate::database::schema::get_migrations;
use crate::utils::error::AppResult;

pub fn initialize_database<R: Runtime>(app: &AppHandle<R>) -> AppResult<()> {
    let db_path = get_db_path(app)?;
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
    let current_version: i32 = conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM __flux_migrations",
        [],
        |row| row.get(0),
    ).unwrap_or(0);

    let migrations = get_migrations();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let tx = conn.transaction()?;
    
    for migration in migrations {
        if migration.version as i32 > current_version {
            println!("[Flux DB] Applying migration {}: {}", migration.version, migration.description);
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

fn ensure_default_settings(conn: &Connection) -> AppResult<()> {
    let defaults = [
        ("auto_indexing", "true"),
        ("scan_frequency", "30"),
        ("hw_acceleration", "true"),
        ("ffmpeg_threading", "Auto"),
        ("watched_threshold", "90"),
        ("video_audio_transition", "true"),
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
