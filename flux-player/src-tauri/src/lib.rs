pub mod commands;
pub mod database;
pub mod scanner;
pub mod utils;

use std::sync::atomic::AtomicUsize;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenvy::dotenv().ok();

    tauri::Builder::default()
        .manage(commands::settings::TmdbState {
            rotation_index: AtomicUsize::new(0),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_sql::Builder::default()
                .add_migrations(database::schema::DB_NAME, database::schema::get_migrations())
                .build()
        )
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init()) // Fixed: Dialog plugin init added
        .invoke_handler(tauri::generate_handler![
            commands::settings::get_computer_name, 
            commands::settings::get_tmdb_key, 
            commands::library::cache_tmdb_image,
            commands::library::get_file_oshash,
            commands::library::start_library_scan,
            utils::folders::get_default_media_folders
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
