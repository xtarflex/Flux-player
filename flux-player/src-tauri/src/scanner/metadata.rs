use serde::{Deserialize, Serialize};

use lofty::prelude::*;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::Path;
use tauri::{AppHandle, Manager, Runtime};

#[cfg(target_os = "windows")]
pub fn extract_native_thumbnail(path: &str, cache_dir: &Path, file_name: &str) -> Option<String> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;
    use windows::core::{ComInterface, PCWSTR};
    use windows::Win32::Foundation::SIZE;
    use windows::Win32::Graphics::Gdi::{
        CreateCompatibleDC, DeleteObject, GetDIBits, BITMAPINFO, BITMAPINFOHEADER, BI_RGB,
        DIB_RGB_COLORS, HBITMAP, HDC,
    };
    use windows::Win32::System::Com::{CoInitializeEx, CoUninitialize, COINIT_APARTMENTTHREADED};
    use windows::Win32::UI::Shell::{
        IShellItem, IShellItemImageFactory, SHCreateItemFromParsingName, SIIGBF_THUMBNAILONLY,
    };

    let path_wide_vec: Vec<u16> = OsStr::new(path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let out_path = cache_dir.join(format!("{}.png", file_name));
    if out_path.exists() {
        return Some(out_path.to_string_lossy().to_string());
    }
    let pcwstr = PCWSTR::from_raw(path_wide_vec.as_ptr());

    // New Hardening: Do not attempt extraction for 0-byte or tiny files (prevents Windows generic icon fallback)
    if let Ok(meta) = std::fs::metadata(path) {
        if meta.len() < 1024 {
            // Smaller than 1KB is almost certainly not a valid movie with a thumbnail
            return None;
        }
    }

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let mut success = false;
        let item_result: windows::core::Result<IShellItem> =
            SHCreateItemFromParsingName(pcwstr, None);
        if let Ok(item) = item_result {
            if let Ok(factory) = item.cast::<IShellItemImageFactory>() {
                // We request a slightly larger square to allow for a high-quality 2:3 portrait crop
                let size = SIZE { cx: 600, cy: 600 };

                let hbitmap_result: windows::core::Result<HBITMAP> =
                    factory.GetImage(size, SIIGBF_THUMBNAILONLY);

                if let Ok(hbitmap) = hbitmap_result {
                    let hdc = CreateCompatibleDC(HDC::default());
                    let mut bmi = BITMAPINFO {
                        bmiHeader: BITMAPINFOHEADER {
                            biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                            biWidth: size.cx,
                            biHeight: -size.cy, // top-down
                            biPlanes: 1,
                            biBitCount: 32,
                            biCompression: BI_RGB.0,
                            ..Default::default()
                        },
                        ..Default::default()
                    };

                    let mut pixels: Vec<u8> = vec![0; (size.cx * size.cy * 4) as usize];
                    let res = GetDIBits(
                        hdc,
                        hbitmap,
                        0,
                        size.cy as u32,
                        Some(pixels.as_mut_ptr() as *mut std::ffi::c_void),
                        &mut bmi,
                        DIB_RGB_COLORS,
                    );

                    if res != 0 {
                        for chunk in pixels.chunks_exact_mut(4) {
                            let b = chunk[0];
                            let r = chunk[2];
                            chunk[0] = r;
                            chunk[2] = b;
                            chunk[3] = 255;
                        }

                        if let Some(mut img) =
                            ::image::ImageBuffer::<::image::Rgba<u8>, Vec<u8>>::from_raw(
                                size.cx as u32,
                                size.cy as u32,
                                pixels,
                            )
                        {
                            let (w, h) = img.dimensions();

                            // Native Auto-Trim Algorithm: Remove Letterboxing
                            let mut top_bound = 0;
                            let mut bottom_bound = h - 1;

                            // 1. Scan Top Down
                            for y in 0..h {
                                let mut row_is_black = true;
                                for x in 0..w {
                                    let pixel = img.get_pixel(x, y);
                                    // Pixel is black if R, G, B < 8
                                    if pixel[0] >= 8 || pixel[1] >= 8 || pixel[2] >= 8 {
                                        row_is_black = false;
                                        break;
                                    }
                                }
                                if !row_is_black {
                                    top_bound = y;
                                    break;
                                }
                            }

                            // 2. Scan Bottom Up
                            for y in (top_bound..h).rev() {
                                let mut row_is_black = true;
                                for x in 0..w {
                                    let pixel = img.get_pixel(x, y);
                                    if pixel[0] >= 8 || pixel[1] >= 8 || pixel[2] >= 8 {
                                        row_is_black = false;
                                        break;
                                    }
                                }
                                if !row_is_black {
                                    bottom_bound = y;
                                    break;
                                }
                            }

                            // 3. Extract Clean Crop
                            let crop_height =
                                (bottom_bound as i32 - top_bound as i32 + 1).max(1) as u32;
                            let cropped_img =
                                ::image::imageops::crop(&mut img, 0, top_bound, w, crop_height)
                                    .to_image();

                            if cropped_img.save(&out_path).is_ok() {
                                success = true;
                            }
                        }
                    }
                    if !hbitmap.is_invalid() {
                        let _ = DeleteObject(hbitmap);
                    }
                }
            }
        }
        CoUninitialize();
        if success {
            return Some(out_path.to_string_lossy().to_string());
        }
    }
    None
}

#[cfg(not(target_os = "windows"))]
pub fn extract_native_thumbnail(
    _path: &str,
    _cache_dir: &Path,
    _file_name: &str,
) -> Option<String> {
    None
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub series_tag: Option<String>,
    pub is_watched: bool,
    pub added_at: u64,
    pub needs_tmdb_scan: bool,
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
    existing_title: Option<String>,
    existing_poster: Option<String>,
    cached_show: Option<MediaMetadata>,
) -> Option<MediaMetadata> {
    let file_stem = path.file_stem()?.to_str()?;

    let (cleaned_title, extracted_year, extracted_series) = clean_media_title(file_stem);

    // BUG FIX: Isolate SEARCH from DISPLAY.
    // display_title: What the user sees (might have ' - S01E01' appended)
    // search_query: What we send to TMDB (Must be the clean show/movie name)
    let display_title = existing_title.clone().unwrap_or(cleaned_title.clone());
    let year = extracted_year;

    // Calculate Search Query:
    // Always fallback to cleaned_title (from filename) if existing_title contains a tag.
    let mut search_query = display_title.clone();
    if let Some(ref tag) = extracted_series {
        if search_query.contains(tag) {
            search_query = cleaned_title.clone(); // Filename cleaning is more reliable for TMDB
        }
    }

    let mut metadata = MediaMetadata {
        path: path.to_string_lossy().to_string(),
        title: display_title,
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
        series_tag: extracted_series.clone(),
        is_watched: false,
        added_at,
        needs_tmdb_scan: false,
    };

    // Attempt to extract embedded artwork and duration from Loft (Core metadata)
    let mut embedded_art = None;
    match lofty::read_from_path(path) {
        Ok(tagged_file) => {
            if let Some(tag) = tagged_file
                .primary_tag()
                .or_else(|| tagged_file.first_tag())
            {
                if let Some(picture) = tag.pictures().first() {
                    embedded_art =
                        cache_album_art(app, picture.data(), None, Some(&metadata.title));
                }
            }
            metadata.duration = Some(tagged_file.properties().duration().as_secs() as u32);
        }
        Err(e) => {
            log::warn!(
                "[Flux Scanner] Core metadata extraction failed (Loft) for {}: {:?}",
                path.display(),
                e
            );
        }
    }

    // Attempt to enrich with TV Show Cache or TMDB
    let mut embed_fallback = false;

    if let Some(cache) = cached_show {
        metadata.title = cache
            .title
            .split(" - ")
            .next()
            .unwrap_or(&cache.title)
            .to_string();
        metadata.poster_path = cache.poster_path;
        metadata.backdrop_path = cache.backdrop_path;
        metadata.genres = cache.genres;
        metadata.director = cache.director;
        metadata.starring = cache.starring;
        metadata.synopsis = cache.synopsis;
        metadata.rating = cache.rating;
        metadata.year = cache.year;

        // BUG FIX: Propagate healing requirement.
        // If the cache itself is an offline/fallback record, the child must also scan later.
        metadata.needs_tmdb_scan = cache.needs_tmdb_scan;
    } else {
        match super::tmdb::search_metadata_advanced(app, &search_query, metadata.year).await {
            Ok(Some((tmdb_meta, api_key))) => {
                metadata.title = tmdb_meta
                    .title
                    .or(tmdb_meta.name)
                    .unwrap_or(metadata.title.clone());

                let mut image_download_failed = false;

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
                        Ok(cached_path) => {
                            // If we found a new TMDB poster and had a fallback one, delete the fallback
                            if let Some(old_poster) = existing_poster {
                                if old_poster.ends_with(".png") && old_poster != cached_path {
                                    let _ = std::fs::remove_file(&old_poster);
                                    println!(
                                        "[Flux Scanner] Deleted old fallback poster: {}",
                                        old_poster
                                    );
                                }
                            }
                            metadata.poster_path = Some(cached_path);
                        }
                        Err(_) => {
                            embed_fallback = true;
                            image_download_failed = true;
                        }
                    }
                } else {
                    embed_fallback = true;
                }

                // Use TMDB Backdrop
                if let Some(backdrop) = tmdb_meta.backdrop_path {
                    let backdrop_url = super::tmdb::get_image_url(&backdrop, "original");
                    match crate::commands::library::cache_tmdb_image(
                        app.clone(),
                        backdrop_url,
                        "backdrops".to_string(),
                    )
                    .await
                    {
                        Ok(cached_path) => metadata.backdrop_path = Some(cached_path),
                        Err(_) => image_download_failed = true,
                    }
                }

                // Fetch Rich Details (Genres, Director, Starring)
                let tmdb_id = tmdb_meta.id;
                let tmdb_media_type = tmdb_meta.media_type.unwrap_or_else(|| "movie".to_string());

                if let Some(details) =
                    super::tmdb::fetch_details(&api_key, tmdb_id, &tmdb_media_type).await
                {
                    metadata.genres = details.genres;
                    metadata.director = details.director;
                    metadata.starring = details.starring;
                }

                metadata.synopsis = tmdb_meta.overview;
                metadata.rating = tmdb_meta.vote_average;

                // Parse Year
                if let Some(date_str) = tmdb_meta.release_date.or(tmdb_meta.first_air_date) {
                    if date_str.len() >= 4 {
                        if let Ok(y) = date_str[0..4].parse::<u32>() {
                            metadata.year = Some(y);
                        }
                    }
                }

                metadata.needs_tmdb_scan = image_download_failed;
            }
            Ok(None) => {
                // 404 Not Found. Never try again.
                embed_fallback = true;
                metadata.needs_tmdb_scan = false;
            }
            Err(e) => {
                // Network Error / 500 / Rate Limit. Mark for retry.
                log::warn!(
                    "[Flux Scanner] TMDB search failed for '{}': {}. Marking for retry.",
                    search_query,
                    e
                );
                embed_fallback = true;
                metadata.needs_tmdb_scan = true;
            }
        }
    }

    // FINAL POLISH: Always ensure the Season/Episode tag is appended to the title
    if let Some(ref tag) = extracted_series {
        if !metadata.title.ends_with(tag) {
            metadata.title = format!("{} - {}", metadata.title, tag);
        }
    }

    if embed_fallback {
        if embedded_art.is_some() {
            metadata.poster_path = embedded_art;
        } else {
            let app_dir = app.path().app_data_dir().unwrap_or_default();
            let cache_dir = app_dir.join("cache").join("images").join("posters");
            let _ = std::fs::create_dir_all(&cache_dir);
            let file_stem_safe = file_stem.replace(|c: char| !c.is_alphanumeric(), "_");
            metadata.poster_path =
                extract_native_thumbnail(&metadata.path, &cache_dir, &file_stem_safe);
        }
    }

    Some(metadata)
}

pub async fn process_audio<R: Runtime>(
    app: &AppHandle<R>,
    path: &std::path::Path,
    added_at: u64,
    existing_title: Option<String>,
    existing_artist: Option<String>,
    existing_album: Option<String>,
) -> Option<MediaMetadata> {
    let mut metadata = MediaMetadata {
        path: path.to_string_lossy().to_string(),
        title: existing_title.unwrap_or_else(|| {
            path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        }),
        year: None,
        artist: existing_artist,
        album: existing_album,
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
        series_tag: None,
        is_watched: false,
        added_at,
        needs_tmdb_scan: false,
    };

    match lofty::read_from_path(path) {
        Ok(tagged_file) => {
            if let Some(tag) = tagged_file
                .primary_tag()
                .or_else(|| tagged_file.first_tag())
            {
                if metadata.title.is_empty() {
                    if let Some(t) = tag.title().map(|s| s.to_string()) {
                        metadata.title = t;
                    }
                }
                if metadata.artist.is_none() {
                    metadata.artist = tag.artist().map(|s| s.to_string());
                }
                if metadata.album.is_none() {
                    metadata.album = tag.album().map(|s| s.to_string());
                }
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
        Err(e) => {
            log::warn!(
                "[Flux Scanner] Audio metadata extraction failed (Loft) for {}: {:?}",
                path.display(),
                e
            );
        }
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

/// A highly optimized multi-stage string cleaner targeting standard "Scene Release" video naming.
/// Returns: (Cleaned Title, Extracted Year, Extracted Series Tag e.g. "S01E01")
pub fn clean_media_title(raw_title: &str) -> (String, Option<u32>, Option<String>) {
    use std::sync::OnceLock;

    // Stage 0: Initial Brackets and prepended resolution tags
    let mut title = raw_title.replace(['_', '.'], " ");

    // Wipe leading tags in brackets like "[1080p] Film Name" or prepended quality like "1080pMovieName"
    static PREPEND_PURGE_RE: OnceLock<regex::Regex> = OnceLock::new();
    let prepend_purge_re = PREPEND_PURGE_RE.get_or_init(|| {
        regex::Regex::new(r"(?i)^(?:\[.*?\]|1080p|720p|4k|2160p|h264|x264|h265|x265|hevc)\s*")
            .unwrap()
    });
    title = prepend_purge_re.replace(&title, "").to_string();

    // Stage 1: Separator normalization
    // If the title contains hyphens and they outnumber spaces, it's likely hyphen-delimited
    let hyphen_count = title.chars().filter(|&c| c == '-').count();
    let space_count = title.chars().filter(|&c| c == ' ').count();
    if hyphen_count > 0 && hyphen_count >= space_count {
        title = title.replace('-', " ");
    }

    // Stage 2: Year Pivot (Golden Rule)
    static YEAR_RE: OnceLock<regex::Regex> = OnceLock::new();
    let year_re = YEAR_RE.get_or_init(|| {
        regex::Regex::new(
            r"(?i)(.*?)(?:[\s\-_]+[\(\[]?((?:19|20|21)(?:\d{2}))[\)\]]?(?:[\s\-_]+|$))",
        )
        .unwrap()
    });

    let mut extracted_year = None;
    if let Some(caps) = year_re.captures(&title) {
        if let (Some(t_match), Some(y_match)) = (caps.get(1), caps.get(2)) {
            extracted_year = y_match.as_str().parse::<u32>().ok();
            title = t_match.as_str().trim().to_string();
        }
    }

    // Stage 3: Series Pivot (Enhanced for multi-episode E01-E02, E01-02, and Part tags)
    static SERIES_RE: OnceLock<regex::Regex> = OnceLock::new();
    let series_re = SERIES_RE.get_or_init(|| {
        regex::Regex::new(r"(?i)^(.*?)(?:[\s\-_]+((?:S\d{1,2}[\s\-_]?E\d{1,4}(?:[\s\-_]?E\d{1,4})?|\d{1,2}x\d{1,4}|Season\s*\d+|E\d{1,2}[\s\-_]?E\d{1,2})))").unwrap()
    });

    let mut extracted_series = None;
    if let Some(caps) = series_re.captures(&title) {
        if let (Some(t_match), Some(s_match)) = (caps.get(1), caps.get(2)) {
            let s_tag = s_match.as_str().trim().to_uppercase();
            title = t_match.as_str().trim().to_string();
            extracted_series = Some(s_tag);
        }
    }

    // Stage 3.5: Fallback for "Show Name Part 1" if not caught above
    static PART_RE: OnceLock<regex::Regex> = OnceLock::new();
    let part_re = PART_RE
        .get_or_init(|| regex::Regex::new(r"(?i)^(.*?)([\s\-_]+(?:Part|Pt)[\s\-_]*\d+)").unwrap());
    if let Some(caps) = part_re.captures(&title) {
        if let (Some(t_match), Some(p_match)) = (caps.get(1), caps.get(2)) {
            let s_tag = p_match.as_str().trim().to_uppercase();
            title = t_match.as_str().trim().to_string();
            extracted_series = Some(s_tag);
        }
    }

    // Stage 4: Tag Purging (Case-insensitive)
    static TAG_RE: OnceLock<regex::Regex> = OnceLock::new();
    let tag_re = TAG_RE.get_or_init(|| {
        regex::Regex::new(r"(?i)\b(1080p|720p|2160p|4k|8k|x264|h264|x265|h265|hevc|bluray|web-dl|webrip|hdrip|cam|ts|aac|dts|ac3|dd5\.1|ddp5\.1|truehd|atmos|extended|unrated|remastered|proper|repack|directors\s*cut|multi|sub|10bit|dual[\s\-]*audio|mkv|mp4|avi|mov|wmv|flv|webm)\b").unwrap()
    });
    title = tag_re.replace_all(&title, "").to_string();

    // Stage 5: Group Drop (Restrictive: Only catch hyphenated or bracketed groups)
    static GROUP_RE: OnceLock<regex::Regex> = OnceLock::new();
    let group_re = GROUP_RE
        .get_or_init(|| regex::Regex::new(r"[\s\-_]+(?:\[[A-Za-z0-9]+\]|-[A-Za-z0-9]+)$").unwrap());
    title = group_re.replace(&title, "").to_string();

    // Final clean space crunch
    static SPACE_RE: OnceLock<regex::Regex> = OnceLock::new();
    let space_re = SPACE_RE.get_or_init(|| regex::Regex::new(r"\s+").unwrap());
    title = space_re.replace_all(&title, " ").to_string();

    (title.trim().to_string(), extracted_year, extracted_series)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_cleaning() {
        // Basic sanitization
        let (title, year, tag) = clean_media_title("The Matrix");
        assert_eq!(title, "The Matrix");
        assert_eq!(year, None);
        assert_eq!(tag, None);

        let (title, _, _) = clean_media_title("The.Matrix");
        assert_eq!(title, "The Matrix");

        let (title, _, _) = clean_media_title("The_Matrix");
        assert_eq!(title, "The Matrix");
    }

    #[test]
    fn test_year_pivot() {
        // Year extraction and pivoting
        let (title, year, tag) = clean_media_title("Inception.2010.1080p.mkv");
        assert_eq!(title, "Inception");
        assert_eq!(year, Some(2010));
        assert_eq!(tag, None);

        let (title, year, _) = clean_media_title("The Batman (2022) Bluray");
        assert_eq!(title, "The Batman");
        assert_eq!(year, Some(2022));

        let (title, year, _) = clean_media_title("Batman [1989]");
        assert_eq!(title, "Batman");
        assert_eq!(year, Some(1989));
    }

    #[test]
    fn test_series_pivot() {
        // Series/Episode tag extraction
        let (title, year, tag) = clean_media_title("The.Office.S01E01.Pilot.mp4");
        assert_eq!(title, "The Office");
        assert_eq!(year, None);
        assert_eq!(tag, Some("S01E01".to_string()));

        let (title, _, tag) = clean_media_title("Succession 1x01");
        assert_eq!(title, "Succession");
        assert_eq!(tag, Some("1X01".to_string()));

        let (title, _, tag) = clean_media_title("The Mandaloiran Season 1");
        assert_eq!(title, "The Mandaloiran");
        assert_eq!(tag, Some("SEASON 1".to_string()));
    }

    #[test]
    fn test_complex_real_world_filenames() {
        // User provided adversarial cases
        let (t, y, s) = clean_media_title("NameRedacted (20xx) S04E04 (2160p MAX WEB-DL Hybrid H265 DV HDR DDP Atmos 5.1 English - HONE).mkv");
        assert_eq!(t, "NameRedacted (20xx)");
        assert_eq!(y, None);
        assert_eq!(s, Some("S04E04".to_string()));

        let (t, _, s) =
            clean_media_title("The.Show.Name.S01E01.720p.HDTV.X264-NoGroup[Proper]-1234567.mkv");
        assert_eq!(t, "The Show Name");
        assert_eq!(s, Some("S01E01".to_string()));

        let (t, y, _) = clean_media_title(
            "Movie.Name.2023.1080p.WEBRip.x265-10bit-MultipleGroups-SceneTeam.mkv",
        );
        assert_eq!(t, "Movie Name");
        assert_eq!(y, Some(2023));

        let (t, _, s) =
            clean_media_title("Show.Name.Part1.E02-E03.Subtitle.Version.3.5.7.1080p-GROUPA.mkv");
        assert_eq!(t, "Show Name"); // Part1 should be stripped by Stage 3.5
        assert_eq!(s, Some("PART1".to_string())); // Current Stage 3.5 captures Part1 into series_tag

        let (t, y, _) = clean_media_title(
            "Some.Film.2022.1080p.BluRay.x264.DualAudio.AAC.2.0.Proper-AnotherGroup[0000000].mkv",
        );
        assert_eq!(t, "Some Film");
        assert_eq!(y, Some(2022));

        let (t, _, s) = clean_media_title(
            "Show_Name_S05_E05_2024_1080p_WEB_DL_H264_10Bit_REPACK_MULTi_SUB[Group1]",
        );
        // Our regex handles underscores now.
        assert_eq!(t, "Show Name");
        assert_eq!(s, Some("S05 E05".to_string()));

        let (t, y, _) =
            clean_media_title("1080pMovieName.2023.1080p.WEBRip.x265-T0T0-GroupA_GroupB.mkv");
        assert_eq!(t, "MovieName"); // 1080p should be purged by Stage 0
        assert_eq!(y, Some(2023));

        let (t, _, s) = clean_media_title(
            "The-Show-Name-S01-E01-E02-2160p-WEB-DL-DD+5.1-HDR-H265-[12345]-GrpName",
        );
        assert_eq!(t, "The Show Name");
        assert_eq!(s, Some("S01 E01 E02".to_string()));

        let (t, y, _) = clean_media_title("[1080p]Film.Name.2022.BluRay.x264-GROUP[rarbg]");
        assert_eq!(t, "Film Name");
        assert_eq!(y, Some(2022));

        let (t, y, _) = clean_media_title("Series.Name.2024.1080p.WEB.H264-MixedGroups.09.11.23");
        assert_eq!(t, "Series Name");
        assert_eq!(y, Some(2024));
        let (t, _, _) = clean_media_title("Guardians of the Galaxy.mp4");
        assert_eq!(t, "Guardians of the Galaxy");

        let (t, _, _) = clean_media_title("Movie.Name.[ReleaseGroup]");
        assert_eq!(t, "Movie Name");
    }
}
