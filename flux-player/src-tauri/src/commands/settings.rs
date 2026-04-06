use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::{AppHandle, Emitter, Runtime, State};
use tauri_plugin_store::StoreExt;
use crate::utils::error::{AppResult, AppError};

pub struct TmdbState {
    pub rotation_index: AtomicUsize,
}

#[tauri::command]
pub async fn get_tmdb_key<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TmdbState>,
    user_key: Option<String>,
) -> AppResult<String> {
    if let Some(key) = user_key {
        if !key.trim().is_empty() {
            return Ok(key);
        }
    }

    // Prioritize SQLite Database (User's Saved Token)
    if let Ok(Some(saved)) = get_setting(app.clone(), "tmdb_read_token".to_string()).await {
        if !saved.trim().is_empty() {
            return Ok(saved);
        }
    }

    let stores = app.store("settings.json").map_err(|e| AppError::Internal(e.to_string()))?;
    let used_count = stores
        .get("tmdb_shared_calls_used")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    if used_count >= 150 {
        // Log to console for debugging
        println!("TMDB API Limit Reached (150/150). Emitting flux-require-api-key event.");
        
        app.emit("flux-require-api-key", ())
            .map_err(|e| AppError::Internal(e.to_string()))?;
        return Err(AppError::ApiLimit("API_LIMIT_REACHED".into()));
    }

    stores.set(
        "tmdb_shared_calls_used",
        serde_json::Value::from(used_count + 1),
    );
    let _ = stores.save(); // Ensure it persists

    let idx = (state.rotation_index.fetch_add(1, Ordering::SeqCst) % 3) + 1;
    let key_var = format!("TMDB_KEY_{}", idx);
    std::env::var(key_var).map_err(|_| AppError::NotFound("API_KEY_NOT_FOUND".into()))
}

#[tauri::command]
pub fn get_computer_name() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "FLUX-PC".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOSTNAME").unwrap_or_else(|_| "FLUX-DEVICE".to_string())
    }
}

#[tauri::command]
pub async fn save_setting<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    key: String,
    value: String,
) -> AppResult<()> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        (key, value),
    )?;

    Ok(())
}

#[tauri::command]
pub async fn get_setting<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    key: String,
) -> AppResult<Option<String>> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    let mut rows = stmt.query((key,))?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}
