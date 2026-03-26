/// Returns the known user directories (Videos, Music) based on the OS environment.
/// On Windows, this reads the user's actual known folder paths via env vars.
/// On macOS/Linux, it uses XDG or common fallback paths.
///
/// # Returns
/// A `Vec<(String, String)>` of `(path, media_type)` tuples.
pub fn get_system_media_folders() -> Vec<(String, String)> {
    let mut folders = Vec::new();

    #[cfg(target_os = "windows")]
    {
        let video_path = std::env::var("USERPROFILE")
            .map(|p| format!("{}\\Videos", p))
            .ok()
            .filter(|p| std::path::Path::new(p).exists());

        let music_path = std::env::var("USERPROFILE")
            .map(|p| format!("{}\\Music", p))
            .ok()
            .filter(|p| std::path::Path::new(p).exists());

        if let Some(p) = video_path { folders.push((p, "video".to_string())); }
        if let Some(p) = music_path { folders.push((p, "audio".to_string())); }
    }

    #[cfg(target_os = "macos")]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        let video = format!("{}/Movies", home);
        let music = format!("{}/Music", home);
        if std::path::Path::new(&video).exists() { folders.push((video, "video".to_string())); }
        if std::path::Path::new(&music).exists() { folders.push((music, "audio".to_string())); }
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_default();
        // Respect XDG_VIDEOS_DIR / XDG_MUSIC_DIR if set
        let video = std::env::var("XDG_VIDEOS_DIR")
            .unwrap_or_else(|_| format!("{}/Videos", home));
        let music = std::env::var("XDG_MUSIC_DIR")
            .unwrap_or_else(|_| format!("{}/Music", home));
        if std::path::Path::new(&video).exists() { folders.push((video, "video".to_string())); }
        if std::path::Path::new(&music).exists() { folders.push((music, "audio".to_string())); }
    }

    folders
}

/// Tauri command: returns the system's known media folder paths.
/// Called on first launch to pre-populate the Storage Settings.
#[tauri::command]
pub fn get_default_media_folders() -> Vec<serde_json::Value> {
    get_system_media_folders()
        .into_iter()
        .map(|(path, media_type)| {
            serde_json::json!({
                "path": path,
                "type": media_type
            })
        })
        .collect()
}
