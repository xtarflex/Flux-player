use serde::{Serialize, Deserialize};
use std::path::Path;
use lofty::prelude::*;
use sha2::{Digest, Sha256};
use std::io::Write;
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub path: String,
    pub title: String,
    pub year: Option<u32>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub album_art_path: Option<String>,
    pub duration: Option<u32>,
    pub media_type: String, // "video", "audio", "mixed"
    pub added_at: u64,
}

pub fn is_video(ext: &str) -> bool {
    matches!(ext, "mp4" | "mkv" | "avi" | "mov" | "webm" | "m4v")
}

pub fn is_audio(ext: &str) -> bool {
    matches!(ext, "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac")
}

pub fn process_video(path: &Path, added_at: u64) -> Option<MediaMetadata> {
    let file_stem = path.file_stem()?.to_str()?;
    let mut title = file_stem.to_string();
    let mut year = None;

    if let Some(caps) = regex::Regex::new(r"\((\d{4})\)").ok()?.captures(file_stem) {
        if let Some(y_str) = caps.get(1) {
            year = y_str.as_str().parse::<u32>().ok();
            title = title.replace(&format!(" ({})", y_str.as_str()), "").trim().to_string();
        }
    }

    Some(MediaMetadata {
        path: path.to_string_lossy().to_string(),
        title,
        year,
        artist: None,
        album: None,
        poster_path: None,
        backdrop_path: None,
        album_art_path: None,
        duration: None,
        media_type: "video".to_string(),
        added_at,
    })
}

pub fn process_audio<R: Runtime>(app: &AppHandle<R>, path: &Path, added_at: u64) -> Option<MediaMetadata> {
    let mut metadata = MediaMetadata {
        path: path.to_string_lossy().to_string(),
        title: path.file_stem()?.to_string_lossy().to_string(),
        year: None,
        artist: None,
        album: None,
        poster_path: None,
        backdrop_path: None,
        album_art_path: None,
        duration: None,
        media_type: "audio".to_string(),
        added_at,
    };

    if let Ok(tagged_file) = lofty::read_from_path(path) {
        if let Some(tag) = tagged_file.primary_tag().or_else(|| tagged_file.first_tag()) {
            if let Some(t) = tag.title().map(|s| s.to_string()) { metadata.title = t; }
            metadata.artist = tag.artist().map(|s| s.to_string());
            metadata.album = tag.album().map(|s| s.to_string());
            metadata.year = tag.year();
            
            if let Some(picture) = tag.pictures().first() {
                if let Some(art_path) = cache_album_art(app, picture.data(), metadata.artist.as_deref(), metadata.album.as_deref()) {
                    metadata.album_art_path = Some(art_path);
                }
            }
        }
        metadata.duration = Some(tagged_file.properties().duration().as_secs() as u32);
    }

    Some(metadata)
}

fn cache_album_art<R: Runtime>(app: &AppHandle<R>, data: &[u8], artist: Option<&str>, album: Option<&str>) -> Option<String> {
    let artist = artist.unwrap_or("Unknown Artist").to_lowercase();
    let album = album.unwrap_or("Unknown Album").to_lowercase();
    
    let mut hasher = Sha256::new();
    hasher.update(artist.as_bytes());
    hasher.update(album.as_bytes());
    let hash = format!("{:x}", hasher.finalize())[..16].to_string();
    
    let app_dir = app.path().app_data_dir().ok()?;
    let art_dir = app_dir.join("cache").join("images").join("album-art");
    if !art_dir.exists() {
        let _ = std::fs::create_dir_all(&art_dir);
    }
    
    let file_path = art_dir.join(format!("{}.jpg", hash));
    if !file_path.exists() {
        if let Ok(mut file) = std::fs::File::create(&file_path) {
            let _ = file.write_all(data);
        }
    }
    
    #[cfg(target_os = "windows")]
    {
        Some(format!("asset://localhost/{}", file_path.to_string_lossy().replace("\\", "/")))
    }
    #[cfg(not(target_os = "windows"))]
    {
        Some(format!("asset://localhost{}", file_path.to_string_lossy()))
    }
}
