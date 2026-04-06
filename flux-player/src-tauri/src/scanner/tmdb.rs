use crate::commands::settings;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug, Deserialize, Serialize)]
pub struct TmdbSearchResult {
    pub id: u32,
    pub title: Option<String>,
    pub name: Option<String>, // for TV shows
    pub media_type: Option<String>, // "movie" or "tv"
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: Option<f32>,
    pub overview: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TmdbDetails {
    pub genres: Vec<String>,
    pub director: Option<String>,
    pub starring: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TmdbFullDetails {
    pub genres: Vec<TmdbGenre>,
    pub credits: Option<TmdbCredits>,
}

#[derive(Debug, Deserialize)]
struct TmdbGenre {
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct TmdbCredits {
    pub cast: Vec<TmdbCast>,
    pub crew: Vec<TmdbCrew>,
}

#[derive(Debug, Deserialize)]
struct TmdbCast {
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct TmdbCrew {
    pub name: String,
    pub job: String,
}

#[derive(Debug, Deserialize)]
struct TmdbSearchResponse {
    pub results: Vec<TmdbSearchResult>,
}

/// Detects if the key is a v3 API Key or a v4 Read Access Token (RAT)
/// and applies the correct authentication to the request.
fn apply_auth(mut req: RequestBuilder, api_key: &str) -> RequestBuilder {
    let api_key = api_key.trim();
    
    // Add default headers to prevent 403s on some TMDB endpoints
    req = req.header("Accept", "application/json");

    if api_key.len() > 100 || api_key.starts_with("ey") {
        // Use v4 Bearer Token
        println!("[TMDB] Applying v4 Bearer Auth");
        req.header("Authorization", format!("Bearer {}", api_key))
    } else {
        // Use v3 API Key Query Param
        println!("[TMDB] Applying v3 API Key Auth");
        req.query(&[("api_key", api_key)])
    }
}

pub async fn search_metadata<R: Runtime>(
    app: &AppHandle<R>,
    query: &str,
    year: Option<u32>,
) -> Option<(TmdbSearchResult, String)> {
    let state = app.state::<settings::TmdbState>();
    let api_key = settings::get_tmdb_key(app.clone(), state, None)
        .await
        .ok()?;

    let client = Client::new();
    let url = "https://api.themoviedb.org/3/search/multi";
    
    let mut req = client.get(url)
        .query(&[("query", query), ("include_adult", "false")]);

    if let Some(y) = year {
        req = req.query(&[("year", &y.to_string())]);
    }

    // Apply Auth (v3 or v4)
    req = apply_auth(req, &api_key);

    let response = req.send().await.ok()?;
    let search_data: TmdbSearchResponse = response.json().await.ok()?;

    let result = search_data.results.into_iter().next()?;
    Some((result, api_key))
}

pub async fn fetch_details(
    api_key: &str,
    tmdb_id: u32,
    media_type: &str, // "movie" or "tv"
) -> Option<TmdbDetails> {
    let client = Client::new();
    let url = format!("https://api.themoviedb.org/3/{}/{}", media_type, tmdb_id);
    
    let mut req = client.get(url)
        .query(&[("append_to_response", "credits")]);

    // Apply Auth (v3 or v4)
    req = apply_auth(req, api_key);

    let response = req.send().await.ok()?;
    let full_details: TmdbFullDetails = response.json().await.ok()?;

    let genres = full_details
        .genres
        .into_iter()
        .take(3)
        .map(|g| g.name)
        .collect();

    let mut director = None;
    let mut starring = None;

    if let Some(credits) = full_details.credits {
        // Find Director
        director = credits
            .crew
            .iter()
            .find(|c| c.job == "Director" || c.job == "Series Director")
            .map(|c| c.name.clone());

        // Get first 3 cast members
        let cast_names: Vec<String> = credits
            .cast
            .into_iter()
            .take(3)
            .map(|c| c.name)
            .collect();
        
        if !cast_names.is_empty() {
            starring = Some(cast_names.join(", "));
        }
    }

    Some(TmdbDetails {
        genres,
        director,
        starring,
    })
}

pub fn get_image_url(path: &str, size: &str) -> String {
    format!("https://image.tmdb.org/t/p/{}{}", size, path)
}
