use crate::utils::error::{AppError, AppResult};
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use tauri::{AppHandle, Emitter, Runtime, State};
use tauri_plugin_store::StoreExt;

pub struct TmdbState {
    pub rotation_index: AtomicUsize,
    pub unsaved_calls: AtomicU64,
}

#[tauri::command]
pub async fn get_tmdb_key<R: Runtime>(
    app: AppHandle<R>,
    state: State<'_, TmdbState>,
    user_key: Option<String>,
) -> AppResult<String> {
    if let Some(key) = user_key {
        if !key.trim().is_empty() {
            return Ok(key);
        }
    }

    // Prioritize SQLite Database (User's Saved Token)
    if let Ok(Some(saved)) = get_setting(app.clone(), "tmdb_read_token".to_string()).await {
        if !saved.trim().is_empty() {
            return Ok(saved);
        }
    }

    let stores = app
        .store("settings.json")
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // --- SMART DAILY RESET LOGIC ---
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let last_reset_date = stores
        .get("tmdb_last_quota_reset_date")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_default();

    let mut used_count = stores
        .get("tmdb_shared_calls_used")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    // --- DAILY RESET CHECK ---
    if today != last_reset_date {
        if used_count >= 130 {
            println!(
                "[Flux Quota] New day reset. Counter was {}. Resetting to 0.",
                used_count
            );
            used_count = 0;
            stores.set("tmdb_shared_calls_used", serde_json::Value::from(0));
        }
        stores.set("tmdb_last_quota_reset_date", serde_json::Value::from(today));
        let _ = stores.save();
    }

    // Increment local buffer
    let unsaved = state.unsaved_calls.fetch_add(1, Ordering::SeqCst) + 1;
    used_count += 1;

    // Check limit
    if used_count >= 150 {
        println!("[Flux Quota] API Limit Reached (150/150). Forced disk save triggered.");
        stores.set(
            "tmdb_shared_calls_used",
            serde_json::Value::from(used_count),
        );
        let _ = stores.save();

        app.emit("flux-require-api-key", ())
            .map_err(|e| AppError::Internal(e.to_string()))?;
        return Err(AppError::ApiLimit("API_LIMIT_REACHED".into()));
    }

    // Only save to disk every 10 calls to reduce I/O pressure
    if unsaved >= 10 {
        state.unsaved_calls.store(0, Ordering::SeqCst);
        stores.set(
            "tmdb_shared_calls_used",
            serde_json::Value::from(used_count),
        );
        let _ = stores.save();
        println!(
            "[Flux Quota] Batch disk sync performed. Counter: {}",
            used_count
        );
    } else {
        // Just update the store's memory state without committing to file
        stores.set(
            "tmdb_shared_calls_used",
            serde_json::Value::from(used_count),
        );
    }

    let idx = (state.rotation_index.fetch_add(1, Ordering::SeqCst) % 3) + 1;
    let key_var = format!("TMDB_KEY_{}", idx);
    std::env::var(key_var).map_err(|_| AppError::NotFound("API_KEY_NOT_FOUND".into()))
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

#[tauri::command]
pub async fn save_setting<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    key: String,
    value: String,
) -> AppResult<()> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        (key, value),
    )?;

    Ok(())
}

#[tauri::command]
pub async fn get_setting<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
    key: String,
) -> AppResult<Option<String>> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(db_path)?;

    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?1")?;
    let mut rows = stmt.query((key,))?;

    if let Some(row) = rows.next()? {
        Ok(Some(row.get(0)?))
    } else {
        Ok(None)
    }
}

/// Diagnostic report structure for bug reporting.
/// Gathers environment, library stats, and app state into a single payload.
#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct DiagnosticReport {
    pub app_version: String,
    pub os: String,
    pub computer_name: String,
    pub library_total: u64,
    pub library_enriched: u64,
    pub library_pending: u64,
    pub offline_mode: bool,
    pub auto_indexing: bool,
    pub has_custom_tmdb_key: bool,
    pub theme: String,
}

/// Gathers a comprehensive diagnostic report for bug submissions.
/// This collects non-sensitive system and app state info.
#[tauri::command]
pub async fn get_diagnostic_report<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> AppResult<DiagnosticReport> {
    let db_path = crate::database::connection::get_db_path(&app)?;
    let conn = rusqlite::Connection::open(&db_path)?;

    // Library stats
    let library_total: u64 = conn
        .query_row("SELECT COUNT(*) FROM media", [], |row| row.get(0))
        .unwrap_or(0);

    let library_enriched: u64 = conn
        .query_row(
            "SELECT COUNT(*) FROM media WHERE needs_tmdb_scan = 0 AND poster_path IS NOT NULL",
            [],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let library_pending = library_total.saturating_sub(library_enriched);

    // Settings
    let offline_mode = get_setting_sync(&conn, "offlineMode").unwrap_or_default() == "true";
    let auto_indexing =
        get_setting_sync(&conn, "auto_indexing").unwrap_or_else(|| "true".to_string()) == "true";
    let has_custom_tmdb_key = get_setting_sync(&conn, "tmdb_read_token")
        .map(|k| !k.trim().is_empty())
        .unwrap_or(false);
    let theme = get_setting_sync(&conn, "theme").unwrap_or_else(|| "Cyber Dark".to_string());

    // OS Info
    let os = {
        #[cfg(target_os = "windows")]
        {
            "Windows".to_string()
        }
        #[cfg(target_os = "macos")]
        {
            "macOS".to_string()
        }
        #[cfg(target_os = "linux")]
        {
            "Linux".to_string()
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            "Unknown".to_string()
        }
    };

    Ok(DiagnosticReport {
        app_version: "V0.2.0".to_string(),
        os,
        computer_name: get_computer_name(),
        library_total,
        library_enriched,
        library_pending,
        offline_mode,
        auto_indexing,
        has_custom_tmdb_key,
        theme,
    })
}

/// Synchronous helper to read a setting from the database connection directly.
fn get_setting_sync(conn: &rusqlite::Connection, key: &str) -> Option<String> {
    conn.query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
        row.get(0)
    })
    .ok()
}

#[derive(serde::Serialize)]
pub struct ScreenshotResult {
    pub path: String,
    pub base64: String,
}

/// Captures a screenshot of the Flux window in a cross-platform way.
/// Returns the file path and base64 encoded image for preview.
#[tauri::command]
pub async fn capture_screenshot<R: tauri::Runtime>(
    app: tauri::AppHandle<R>,
) -> AppResult<ScreenshotResult> {
    use base64::{engine::general_purpose, Engine as _};
    use std::io::Cursor;
    use tauri::Manager;
    use xcap::Window;

    // Get current process ID for exact targeting
    let current_pid = std::process::id();

    // Find the Flux window belonging to THIS process
    let windows = Window::all().map_err(|e| AppError::Internal(e.to_string()))?;

    println!("--- [Flux Diagnostic] Available Windows ---");
    let mut best_window = None;
    let mut best_score = -1;

    for w in windows {
        let pid = w.pid().unwrap_or(0);
        let title = w.title().unwrap_or_default();
        let app_name = w.app_name().unwrap_or_default();
        let is_min = w.is_minimized().unwrap_or(false);
        let id = w.id().unwrap_or(0);

        println!(
            "Window found: ID={}, PID={}, Title='{}', App='{}', Minimized={}",
            id, pid, title, app_name, is_min
        );

        let title_lower = title.to_lowercase();
        let app_lower = app_name.to_lowercase();

        let pid_match = pid == current_pid;
        let title_match = title_lower.contains("flux") || app_lower.contains("flux");

        if pid_match || title_match {
            let mut score = 0;
            if !is_min {
                score += 20;
            }
            if pid_match {
                score += 10;
            }
            if title_match {
                score += 5;
            }

            if score > best_score {
                best_score = score;
                best_window = Some(w);
            }
        }
    }
    println!("-------------------------------------------");

    let flux_window =
        best_window.ok_or_else(|| AppError::NotFound("Flux window not found".into()))?;

    let id = flux_window.id().unwrap_or(0);
    let title = flux_window.title().unwrap_or_default();
    println!(
        "[Flux Diagnostic] Selected Window: ID={}, Title='{}', PID={:?}",
        id,
        title,
        flux_window.pid().unwrap_or(0)
    );

    // Capture the window
    let image = flux_window
        .capture_image()
        .map_err(|e| AppError::Internal(format!("Failed to capture window image: {}", e)))?;

    // Prepare filename
    let now = chrono::Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = format!("flux_screenshot_{}.png", now);

    // --- Path 1: Internal App Cache ---
    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let screenshots_dir = cache_dir.join("screenshots");
    std::fs::create_dir_all(&screenshots_dir)?;
    let internal_save_path = screenshots_dir.join(&filename);
    image
        .save(&internal_save_path)
        .map_err(|e| AppError::Internal(format!("Failed internal save: {}", e)))?;

    // --- Path 2: Public Pictures Folder ---
    let picture_dir = app
        .path()
        .picture_dir()
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let reports_dir = picture_dir.join("Flux_Reports");
    std::fs::create_dir_all(&reports_dir)?;
    let public_save_path = reports_dir.join(&filename);

    // Attempt public save, but don't crash if it fails (e.g. permission denied)
    let _ = image.save(&public_save_path);

    // Convert to base64 for preview
    let mut buffer = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut buffer), image::ImageFormat::Png)
        .map_err(|e| AppError::Internal(e.to_string()))?;
    let base64_image = general_purpose::STANDARD.encode(buffer);

    Ok(ScreenshotResult {
        path: public_save_path.to_string_lossy().to_string(),
        base64: format!("data:image/png;base64,{}", base64_image),
    })
}

/// Sends a feedback report directly to the developer's Formspree endpoint.
/// Includes feedback text, diagnostic info, and optional screenshot.
#[tauri::command]
pub async fn send_feedback_report(
    feedback: String,
    report: DiagnosticReport,
    screenshot_path: Option<String>,
) -> AppResult<()> {
    use reqwest::multipart;
    use serde_json::json;

    let formspree_url = "https://formspree.io/f/mjgpqrrp";

    // Format the message body with diagnostics
    let diag_table = format!(
        "\n\n---\n## Diagnostic Info\n- OS: {}\n- Version: {}\n- Computer: {}\n- Library: {} ({} enriched)\n- Offline: {}\n- Theme: {}\n",
        report.os, report.app_version, report.computer_name, report.library_total, report.library_enriched, report.offline_mode, report.theme
    );
    let full_message = format!("{}\n{}", feedback, diag_table);

    let client = reqwest::Client::builder()
        .user_agent("Flux-App/0.2.0 (Tauri; Beta)")
        .build()
        .map_err(|e| AppError::Internal(e.to_string()))?;

    // --- Primary Attempt: Multipart (with screenshot) ---
    if let Some(path) = screenshot_path {
        let mut form = multipart::Form::new()
            .text("message", full_message.clone())
            .text("_subject", format!("Flux Beta Feedback - {}", report.os));

        if let Ok(file_bytes) = std::fs::read(&path) {
            let part = multipart::Part::bytes(file_bytes)
                .file_name("screenshot.png")
                .mime_str("image/png")
                .map_err(|e| AppError::Internal(e.to_string()))?;
            form = form.part("screenshot", part);

            let res = client.post(formspree_url).multipart(form).send().await;

            if let Ok(response) = res {
                if response.status().is_success() {
                    return Ok(());
                }
                println!(
                    "[Feedback] Multipart failed ({}). Falling back to text-only.",
                    response.status()
                );
            }
        }
    }

    // --- Fallback Attempt: JSON (No screenshot) ---
    // If multipart failed or wasn't possible, send text-only report.
    let json_body = json!({
        "message": full_message,
        "_subject": format!("Flux Beta Feedback (Text-Only) - {}", report.os)
    });

    let json_res = client
        .post(formspree_url)
        .json(&json_body)
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Direct Send Failed: {}", e)))?;

    if json_res.status().is_success() {
        Ok(())
    } else {
        let err_text = json_res.text().await.unwrap_or_default();
        Err(AppError::Internal(format!(
            "Formspree rejected report: {}",
            err_text
        )))
    }
}
