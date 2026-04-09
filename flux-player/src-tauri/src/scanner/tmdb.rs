use crate::commands::settings;
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, Runtime};

#[derive(Debug, Deserialize, Serialize)]
pub struct TmdbSearchResult {
    pub id: u32,
    pub title: Option<String>,
    pub name: Option<String>,       // for TV shows
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
    search_metadata_advanced(app, query, year)
        .await
        .ok()
        .flatten()
}

// Advanced search returning Result to differentiate Network Error (Err) from Not Found (Ok(None))
pub async fn search_metadata_advanced<R: Runtime>(
    app: &AppHandle<R>,
    query: &str,
    year: Option<u32>,
) -> Result<Option<(TmdbSearchResult, String)>, String> {
    let state = app.state::<settings::TmdbState>();
    let api_key = match settings::get_tmdb_key(app.clone(), state, None).await {
        Ok(key) => key,
        Err(_) => return Err("API_KEY_ERROR".into()),
    };

    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?;
    let url = "https://api.themoviedb.org/3/search/multi";

    let mut req = client
        .get(url)
        .query(&[("query", query), ("include_adult", "false")]);

    if let Some(y) = year {
        req = req.query(&[("year", &y.to_string())]);
    }

    let req = apply_auth(req, &api_key);

    match req.send().await {
        Ok(response) => {
            if response.status().is_client_error() || response.status().is_server_error() {
                return Err(format!("HTTP_ERROR: {}", response.status()));
            }
            if let Ok(search_data) = response.json::<TmdbSearchResponse>().await {
                let result = search_data.results.into_iter().next();
                Ok(result.map(|r| (r, api_key)))
            } else {
                Err("JSON_PARSE_ERROR".into())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}

pub async fn fetch_details(
    api_key: &str,
    tmdb_id: u32,
    media_type: &str, // "movie" or "tv"
) -> Option<TmdbDetails> {
    let client = Client::new();
    let url = format!("https://api.themoviedb.org/3/{}/{}", media_type, tmdb_id);

    let mut req = client.get(url).query(&[("append_to_response", "credits")]);

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
        let cast_names: Vec<String> = credits.cast.into_iter().take(3).map(|c| c.name).collect();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v4_bearer_token_detection() {
        let client = reqwest::Client::new();
        let rat = "ey-very-long-token-that-represents-a-v4-read-access-token-which-starts-with-ey";
        let req = client.get("https://api.themoviedb.org/3/movie/550");
        let req = apply_auth(req, rat);
        let request = req.build().unwrap();

        // Check if Authorization header is set correctly
        let auth_header = request.headers().get("Authorization");
        assert!(
            auth_header.is_some(),
            "Authorization header should be set for v4 tokens"
        );
        assert_eq!(
            auth_header.unwrap().to_str().unwrap(),
            format!("Bearer {}", rat),
            "Authorization header mismatch"
        );
    }

    #[test]
    fn test_v3_api_key_detection() {
        let client = reqwest::Client::new();
        let api_key = "dc88def0a6432a3f1cd3cf80b22e98f6"; // Typical v3 key
        let req = client.get("https://api.themoviedb.org/3/movie/550");
        let req = apply_auth(req, api_key);
        let request = req.build().unwrap();

        // Check if api_key is in search query
        let query = request
            .url()
            .query()
            .expect("URL should have query parameters");
        assert!(
            query.contains("api_key="),
            "Search query should contain api_key"
        );
        assert!(
            query.contains(api_key),
            "Search query should contain the actual key"
        );

        // Verify Authorization header is NOT set
        assert!(
            request.headers().get("Authorization").is_none(),
            "Authorization header should not be set for v3 keys"
        );
    }

    #[test]
    fn test_auth_trimming() {
        let client = reqwest::Client::new();
        let api_key = "  dc88def0a6432a3f1cd3cf80b22e98f6  "; // Messy whitespace
        let req = client.get("https://api.themoviedb.org/3/movie/550");
        let req = apply_auth(req, api_key);
        let request = req.build().unwrap();

        let query = request.url().query().unwrap();
        assert!(
            query.contains("dc88def0a6432a3f1cd3cf80b22e98f6"),
            "Should trim whitespace before applying"
        );
    }

    #[tokio::test]
    async fn smoke_test_v4_auth_real() {
        dotenvy::dotenv().ok();
        // Look for the Read Access Token in .env
        let rat = std::env::var("TMDB_RAT")
            .expect("TMDB_RAT not found in .env. Please add it to test live v4 auth.");

        let client = reqwest::Client::new();
        let url = "https://api.themoviedb.org/3/authentication";
        let req = client.get(url);
        let req = apply_auth(req, &rat);

        let response = req.send().await.expect("Failed to reach TMDB servers");

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        println!("[Flux Smoke Test] TMDB Response: {}", body);

        assert_eq!(
            status, 200,
            "Live v4 Authentication failed! TMDB returned status: {}. Body: {}",
            status, body
        );
        println!("[Flux Smoke Test] Live v4 Authentication successful!");
    }
}
