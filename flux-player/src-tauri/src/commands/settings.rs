use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::{AppHandle, Emitter, Runtime, State};
use tauri_plugin_store::StoreExt;

pub struct TmdbState {
    pub rotation_index: AtomicUsize,
}

#[tauri::command]
pub async fn get_tmdb_key<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TmdbState>,
    user_key: Option<String>,
) -> Result<String, String> {
    if let Some(key) = user_key {
        if !key.trim().is_empty() {
            return Ok(key);
        }
    }

    let stores = app.store("settings.json").map_err(|e| e.to_string())?;
    let used_count = stores.get("tmdb_shared_calls_used").and_then(|v| v.as_u64()).unwrap_or(0);

    if used_count >= 150 {
        app.emit("flux-require-api-key", ()).map_err(|e| e.to_string())?;
        return Err("API_LIMIT_REACHED".to_string());
    }

    stores.set("tmdb_shared_calls_used", serde_json::Value::from(used_count + 1));
    
    let idx = (state.rotation_index.fetch_add(1, Ordering::SeqCst) % 3) + 1;
    let key_var = format!("TMDB_KEY_{}", idx);
    std::env::var(key_var).map_err(|_| "API_KEY_NOT_FOUND".to_string())
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
