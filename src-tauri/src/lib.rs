mod catalog;
mod commands;
mod error;
mod github;
mod http;
mod install;
mod jobs;
mod launch;
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
        .plugin(tauri_plugin_process::init())
        // Mises à jour du launcher : vérifiées par l'interface, installées seulement à la demande du joueur.
        .plugin(tauri_plugin_updater::Builder::new().build())
        .setup(|app| {
            let paths = app.path();
            let state = state::AppState::new(
                paths.app_cache_dir()?,
                paths.app_local_data_dir()?.join("games"),
                paths.home_dir()?,
            )?;
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::catalog::get_catalog,
            commands::catalog::get_game_versions,
            commands::catalog::get_game_icon,
            commands::install::get_installed,
            commands::install::install_version,
            commands::install::cancel_install,
            commands::install::uninstall_version,
            commands::play::launch_game,
            commands::play::get_running,
            commands::play::open_install_dir,
            commands::play::open_saves_dir,
        ])
        .run(tauri::generate_context!())
        .expect("impossible de démarrer EternalLauncher");
}
