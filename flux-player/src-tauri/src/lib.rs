// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::{AppHandle, Emitter, Manager, Runtime, State};
use tauri_plugin_store::StoreExt;

const TMDB_KEYS: [&str; 3] = [
    "YOUR_SHARED_KEY_1",
    "YOUR_SHARED_KEY_2",
    "YOUR_SHARED_KEY_3",
];

struct TmdbState {
    rotation_index: AtomicUsize,
}

#[tauri::command]
async fn get_tmdb_key<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TmdbState>,
    user_key: Option<String>,
) -> Result<String, String> {
    // If user provided a custom key, always use it
    if let Some(key) = user_key {
        if !key.trim().is_empty() {
            return Ok(key);
        }
    }

    // Check share limit in store
    let stores = app.store("settings.json").map_err(|e| e.to_string())?;
    let used_count = stores.get("tmdb_shared_calls_used").and_then(|v| v.as_u64()).unwrap_or(0);

    if used_count >= 150 {
        // Hard stop: emit window event to Frontend
        app.emit("flux-require-api-key", ()).map_err(|e| e.to_string())?;
        return Err("API_LIMIT_REACHED".to_string());
    }

    // Increment used count in store
    stores.set("tmdb_shared_calls_used", serde_json::Value::from(used_count + 1));
    // Note: In real app, you'd want to save explicitly if the plugin doesn't auto-save (StoreExt.save() is required after set usually)
    // stores.save(); // Not needed if you set it to auto-save, but let's check store API

    // Rotate and return shared key
    let idx = state.rotation_index.fetch_add(1, Ordering::SeqCst) % TMDB_KEYS.len();
    Ok(TMDB_KEYS[idx].to_string())
}
use std::io::Write;
use std::path::PathBuf;
mod oshash;

#[tauri::command]
fn get_file_oshash(path: String) -> Result<String, String> {
    oshash::compute_oshash(&path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn cache_tmdb_image<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    image_type: String, // "posters" or "backdrops" or "album-art"
) -> Result<String, String> {
    if url.is_empty() {
        return Err("EMPTY_URL".to_string());
    }

    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let cache_dir = app_dir.join("cache").join("images").join(image_type);

    if !cache_dir.exists() {
        std::fs::create_dir_all(&cache_dir).map_err(|e| e.to_string())?;
    }

    // Use a hash of the URL as filename to avoid duplicates
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

    // Return the local asset:// path
    // Tauri 2 handles asset protocol slightly differently, but it usually follows asset://localhost/<path>
    #[cfg(target_os = "windows")]
    {
        Ok(format!("asset://localhost/{}", target_path.to_string_lossy().replace("\\", "/")))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Ok(format!("asset://localhost{}", target_path.to_string_lossy()))
    }
}
#[tauri::command]
fn get_computer_name() -> String {
    #[cfg(target_os = "windows")]
    {
        std::env::var("COMPUTERNAME").unwrap_or_else(|_| "FLUX-PC".to_string())
    }
    #[cfg(not(target_os = "windows"))]
    {
        std::env::var("HOSTNAME").unwrap_or_else(|_| "FLUX-DEVICE".to_string())
    }
}

use tauri_plugin_sql::{Migration, MigrationKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let migrations = vec![
        Migration {
            version: 1,
            description: "create media and playlists tables",
            sql: "
                CREATE TABLE media (
                    path TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    year INTEGER,
                    artist TEXT,
                    album TEXT,
                    poster_path TEXT,
                    backdrop_path TEXT,
                    album_art_path TEXT,
                    duration INTEGER,
                    media_type TEXT CHECK(media_type IN ('video', 'audio', 'mixed')),
                    last_played INTEGER DEFAULT 0,
                    added_at INTEGER NOT NULL
                );
                CREATE TABLE playlists (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    description TEXT,
                    type TEXT CHECK(type IN ('audio', 'video', 'mixed')),
                    created_at INTEGER NOT NULL,
                    updated_at INTEGER NOT NULL,
                    is_smart BOOLEAN DEFAULT 0,
                    smart_criteria TEXT,
                    cover_image TEXT
                );
                CREATE TABLE playlist_items (
                    playlist_id TEXT NOT NULL,
                    media_path TEXT NOT NULL,
                    position INTEGER NOT NULL,
                    added_at INTEGER NOT NULL,
                    FOREIGN KEY (playlist_id) REFERENCES playlists(id) ON DELETE CASCADE,
                    FOREIGN KEY (media_path) REFERENCES media(path) ON DELETE CASCADE,
                    PRIMARY KEY (playlist_id, media_path)
                );
            ",
            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .manage(TmdbState {
            rotation_index: AtomicUsize::new(0),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations("sqlite:flux.db", migrations)
                .build()
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_computer_name, 
            get_tmdb_key, 
            cache_tmdb_image,
            get_file_oshash
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
