use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryItem {
    pub path: String,
    pub title: String,
    pub year: Option<u32>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub album_art_path: Option<String>,
    pub duration: Option<u32>,
    pub media_type: String,
    pub last_played: i64,
    pub added_at: i64,
    pub synopsis: Option<String>,
    pub rating: Option<f64>,
    pub genres: Vec<String>,
    pub director: Option<String>,
    pub starring: Option<String>,
    pub series_tag: Option<String>,
    pub is_watched: bool,
    pub last_position: i64,
    pub is_favorite: bool,
}
