use crate::commands::settings;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug, Deserialize, Serialize)]
pub struct TmdbSearchResult {
    pub id: u32,
    pub title: Option<String>,
    pub name: Option<String>, // for TV shows
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: Option<f32>,
    pub overview: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TmdbSearchResponse {
    pub results: Vec<TmdbSearchResult>,
}

pub async fn search_metadata<R: Runtime>(
    app: &AppHandle<R>,
    query: &str,
    year: Option<u32>,
) -> Option<TmdbSearchResult> {
    let state = app.state::<settings::TmdbState>();
    let api_key = settings::get_tmdb_key(app.clone(), state, None)
        .await
        .ok()?;

    let client = Client::new();
    let mut url = format!(
        "https://api.themoviedb.org/3/search/multi?api_key={}&query={}&include_adult=false",
        api_key,
        urlencoding::encode(query)
    );

    if let Some(y) = year {
        url.push_str(&format!("&year={}", y));
    }

    let response = client.get(url).send().await.ok()?;
    let search_data: TmdbSearchResponse = response.json().await.ok()?;

    search_data.results.into_iter().next() // Return the first match
}

pub fn get_image_url(path: &str, size: &str) -> String {
    format!("https://image.tmdb.org/t/p/{}{}", size, path)
}
