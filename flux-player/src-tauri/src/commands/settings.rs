use std::sync::atomic::{AtomicUsize, AtomicU64, Ordering};
use tauri::{AppHandle, Emitter, Runtime, State};
use tauri_plugin_store::StoreExt;
use crate::utils::error::{AppResult, AppError};

pub struct TmdbState {
    pub rotation_index: AtomicUsize,
    pub unsaved_calls: AtomicU64,
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
    
    // --- SMART DAILY RESET LOGIC ---
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let last_reset_date = stores.get("tmdb_last_quota_reset_date")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    let mut used_count = stores
        .get("tmdb_shared_calls_used")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    // --- DAILY RESET CHECK ---
    if today != last_reset_date {
        if used_count >= 130 {
            println!("[Flux Quota] New day reset. Counter was {}. Resetting to 0.", used_count);
            used_count = 0;
            stores.set("tmdb_shared_calls_used", serde_json::Value::from(0));
        }
        stores.set("tmdb_last_quota_reset_date", serde_json::Value::from(today));
        let _ = stores.save();
    }

    // Increment local buffer
    let unsaved = state.unsaved_calls.fetch_add(1, Ordering::SeqCst) + 1;
    used_count += 1;

    // Check limit
    if used_count >= 150 {
        println!("[Flux Quota] API Limit Reached (150/150). Forced disk save triggered.");
        stores.set("tmdb_shared_calls_used", serde_json::Value::from(used_count));
        let _ = stores.save();
        
        app.emit("flux-require-api-key", ())
            .map_err(|e| AppError::Internal(e.to_string()))?;
        return Err(AppError::ApiLimit("API_LIMIT_REACHED".into()));
    }

    // Only save to disk every 10 calls to reduce I/O pressure
    if unsaved >= 10 {
        state.unsaved_calls.store(0, Ordering::SeqCst);
        stores.set("tmdb_shared_calls_used", serde_json::Value::from(used_count));
        let _ = stores.save();
        println!("[Flux Quota] Batch disk sync performed. Counter: {}", used_count);
    } else {
        // Just update the store's memory state without committing to file
        stores.set("tmdb_shared_calls_used", serde_json::Value::from(used_count));
    }

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
