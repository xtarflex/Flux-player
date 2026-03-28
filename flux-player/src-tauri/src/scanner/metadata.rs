use serde::{Deserialize, Serialize};

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
    pub synopsis: Option<String>,
    pub rating: Option<f32>,
    pub genres: Vec<String>,
    pub director: Option<String>,
    pub starring: Option<String>,
    pub added_at: u64,
}

pub fn is_video(ext: &str) -> bool {
    matches!(ext, "mp4" | "mkv" | "avi" | "mov" | "webm" | "m4v")
}

pub fn is_audio(ext: &str) -> bool {
    matches!(ext, "mp3" | "flac" | "wav" | "ogg" | "m4a" | "aac")
}

pub async fn process_video<R: Runtime>(
    app: &AppHandle<R>,
    path: &std::path::Path,
    added_at: u64,
) -> Option<MediaMetadata> {
    let file_stem = path.file_stem()?.to_str()?;
    let mut title = file_stem.to_string();
    let mut year = None;

    if let Some(caps) = regex::Regex::new(r"\((\d{4})\)").ok()?.captures(file_stem) {
        if let Some(y_str) = caps.get(1) {
            year = y_str.as_str().parse::<u32>().ok();
            title = title
                .replace(&format!(" ({})", y_str.as_str()), "")
                .trim()
                .to_string();
        }
    }

    let mut metadata = MediaMetadata {
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
        synopsis: None,
        rating: None,
        genres: Vec::new(),
        director: None,
        starring: None,
        added_at,
    };

    // Attempt to extract embedded artwork from Video (similar to audio but limited formats) - This is our BACKUP
    let mut embedded_art = None;
    if let Ok(tagged_file) = lofty::read_from_path(path) {
        if let Some(tag) = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag())
        {
            if let Some(picture) = tag.pictures().first() {
                embedded_art = cache_album_art(app, picture.data(), None, Some(&metadata.title));
            }
        }
        metadata.duration = Some(tagged_file.properties().duration().as_secs() as u32);
    }

    // Attempt to enrich with TMDB - This is our MAIN
    if let Some((tmdb_meta, api_key)) = super::tmdb::search_metadata(app, &metadata.title, metadata.year).await
    {
        metadata.title = tmdb_meta.title.or(tmdb_meta.name).unwrap_or(metadata.title);

        // Use TMDB Poster if available
        if let Some(poster) = tmdb_meta.poster_path {
            let poster_url = super::tmdb::get_image_url(&poster, "w500");
            match crate::commands::library::cache_tmdb_image(
                app.clone(),
                poster_url,
                "posters".to_string(),
            )
            .await
            {
                Ok(cached_path) => metadata.poster_path = Some(cached_path),
                Err(_) => metadata.poster_path = embedded_art,
            }
        } else {
            metadata.poster_path = embedded_art;
        }

        // Use TMDB Backdrop
        if let Some(backdrop) = tmdb_meta.backdrop_path {
            let backdrop_url = super::tmdb::get_image_url(&backdrop, "original");
            metadata.backdrop_path = crate::commands::library::cache_tmdb_image(
                app.clone(),
                backdrop_url,
                "backdrops".to_string(),
            )
            .await
            .ok();
        }

        // Fetch Rich Details (Genres, Director, Starring) — reuse same api_key, no extra quota spend
        let tmdb_id = tmdb_meta.id;
        let tmdb_media_type = tmdb_meta.media_type.unwrap_or_else(|| "movie".to_string());
        
        if let Some(details) = super::tmdb::fetch_details(&api_key, tmdb_id, &tmdb_media_type).await {
            metadata.genres = details.genres;
            metadata.director = details.director;
            metadata.starring = details.starring;
        }

        metadata.synopsis = tmdb_meta.overview;
        metadata.rating = tmdb_meta.vote_average;
    } else {
        // Fallback to embedded art if TMDB search fails or returns nothing
        metadata.poster_path = embedded_art;
    }

    Some(metadata)
}

pub async fn process_audio<R: Runtime>(
    app: &AppHandle<R>,
    path: &std::path::Path,
    added_at: u64,
) -> Option<MediaMetadata> {
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
        synopsis: Some("Audio track.".to_string()),
        rating: None,
        genres: Vec::new(),
        director: None,
        starring: None,
        added_at,
    };

    if let Ok(tagged_file) = lofty::read_from_path(path) {
        if let Some(tag) = tagged_file
            .primary_tag()
            .or_else(|| tagged_file.first_tag())
        {
            if let Some(t) = tag.title().map(|s| s.to_string()) {
                metadata.title = t;
            }
            metadata.artist = tag.artist().map(|s| s.to_string());
            metadata.album = tag.album().map(|s| s.to_string());
            metadata.year = tag.year();

            if let Some(picture) = tag.pictures().first() {
                if let Some(art_path) = cache_album_art(
                    app,
                    picture.data(),
                    metadata.artist.as_deref(),
                    metadata.album.as_deref(),
                ) {
                    metadata.album_art_path = Some(art_path.clone());
                    metadata.poster_path = Some(art_path);
                }
            }
        }
        metadata.duration = Some(tagged_file.properties().duration().as_secs() as u32);
    }

    Some(metadata)
}

fn cache_album_art<R: Runtime>(
    app: &AppHandle<R>,
    data: &[u8],
    artist: Option<&str>,
    album: Option<&str>,
) -> Option<String> {
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

    Some(file_path.to_string_lossy().to_string())
}
