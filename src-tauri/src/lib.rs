mod catalog;
mod commands;
mod error;
mod github;
mod http;
mod platform;
mod state;
mod version;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Une seule fenêtre du launcher : un second lancement ramène la première au premier plan.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let cache_dir = app.path().app_cache_dir()?;
            app.manage(state::AppState::new(cache_dir)?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_catalog,
            commands::get_game_versions,
            commands::get_game_icon,
        ])
        .run(tauri::generate_context!())
        .expect("impossible de démarrer EternalLauncher");
}
