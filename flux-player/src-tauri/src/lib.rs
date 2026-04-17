pub mod commands;
pub mod database;
pub mod scanner;
pub mod utils;

use tauri::Emitter;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::LogDir {
                        file_name: Some("flux.log".to_string()),
                    }),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Webview),
                ])
                .level(log::LevelFilter::Info)
                .build(),
        )
        .plugin(tauri_plugin_fs::init())
        .manage(commands::settings::TmdbState {
            rotation_index: std::sync::atomic::AtomicUsize::new(0),
            unsaved_calls: std::sync::atomic::AtomicU64::new(0),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(
                    database::schema::DB_NAME,
                    database::schema::get_migrations(),
                )
                .build(),
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_computer_name,
            commands::settings::get_tmdb_key,
            commands::settings::save_setting,
            commands::settings::get_setting,
            commands::library::cache_tmdb_image,
            commands::library::get_file_oshash,
            commands::library::start_library_scan,
            commands::library::heal_library,
            commands::library::refresh_media_metadata,
            commands::library::update_media_field,
            commands::library::save_playback_progress,
            commands::library::get_playback_progress,
            commands::library::get_all_media,
            commands::library::toggle_favorite_status,
            commands::library::toggle_media_watched_status,
            commands::settings::get_diagnostic_report,
            commands::settings::capture_screenshot,
            commands::settings::send_feedback_report,
            commands::settings::get_storage_stats,
            utils::folders::get_default_media_folders,
            commands::audio::get_system_mute_status,
            commands::audio::get_current_audio_device,
            commands::settings::show_hud,
            commands::settings::factory_reset,
            commands::settings::open_uninstaller,
        ])
        .setup(|app| {
            // ── Database Initialization (Sequential) ─────────────────────────
            // We run this before any background tasks to ensure schema exists.
            if let Err(e) = crate::database::init::initialize_database(app.handle()) {
                eprintln!("[Flux DB] Critical initialization failure: {:?}", e);
            }

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Err(e) = background_scan_task(&handle).await {
                        eprintln!("[Flux Background Scan] Task execution failed: {:?}", e);
                    }
                    // Check every 30 seconds for scheduled scans
                    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn background_scan_task<R: tauri::Runtime>(
    app: &tauri::AppHandle<R>,
) -> crate::utils::error::AppResult<()> {
    // 1. Fetch current settings from database
    let auto_indexing =
        crate::commands::settings::get_setting(app.clone(), "auto_indexing".to_string())
            .await?
            .map(|v| v == "true")
            .unwrap_or(true);

    if !auto_indexing {
        return Ok(());
    }

    let scan_frequency_mins =
        crate::commands::settings::get_setting(app.clone(), "scan_frequency".to_string())
            .await?
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(30);

    let last_scan_time =
        crate::commands::settings::get_setting(app.clone(), "last_library_scan_time".to_string())
            .await?
            .and_then(|v| v.parse::<u64>().ok())
            .unwrap_or(0);

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // 2. Determine if enough time has passed
    if now - last_scan_time >= scan_frequency_mins * 60 {
        let folders_json =
            crate::commands::settings::get_setting(app.clone(), "library_folders".to_string())
                .await?
                .unwrap_or_else(|| "[]".to_string());

        let folders: Vec<serde_json::Value> =
            serde_json::from_str(&folders_json).unwrap_or_default();

        if !folders.is_empty() {
            println!(
                "[Flux Background Scan] Starting scheduled scan of {} locations.",
                folders.len()
            );

            // Phase 3: Healing Sync - Heal the backlog FIRST (once per background scan cycle)
            let _ = crate::scanner::healing_sync(app.clone()).await;

            for folder in folders {
                if let Some(path) = folder.get("path").and_then(|p| p.as_str()) {
                    // Reuse the existing scan command logic
                    if let Err(e) =
                        crate::commands::library::start_library_scan(app.clone(), path.to_string())
                            .await
                    {
                        eprintln!(
                            "[Flux Background Scan] Error scanning path '{}': {:?}",
                            path, e
                        );
                    }
                }
            }

            // Update timestamp and notify frontend
            crate::commands::settings::save_setting(
                app.clone(),
                "last_library_scan_time".to_string(),
                now.to_string(),
            )
            .await?;
            app.emit("flux-library-updated", ()).ok();
            println!("[Flux Background Scan] Scheduled scan complete.");
        }
    }

    Ok(())
}
