use crate::error::AppError;
use crate::install::paths::GamePaths;
use crate::install::registry;
use crate::launch::command::LaunchCommand;
use crate::launch::saves::saves_dir;
use crate::platform::Platform;
use crate::state::AppState;
use std::path::Path;
use tauri::{AppHandle, Emitter, Manager, State};

/// Lance une version installée. Le launcher reste ouvert et suit le jeu jusqu'à sa fermeture.
#[tauri::command]
pub async fn launch_game(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
    tag: String,
    variant: Option<String>,
) -> Result<(), AppError> {
    let (game, spec) = state.game_for_platform(&id).await?;
    let paths = GamePaths::new(&state.install_root, &game.id);
    if !registry::is_installed(&paths, &tag)? {
        return Err(AppError::NotInstalled);
    }
    let command = LaunchCommand::build(&spec, &paths.version(&tag)?, variant.as_deref())?;
    state.jobs.start_game(&game.id)?;
    let mut child = match command.spawn() {
        Ok(child) => child,
        Err(error) => {
            state.jobs.finish_game(&game.id);
            return Err(error);
        }
    };
    // Sert à prévenir si l'on relance ensuite une version plus ancienne.
    let _ = registry::set_last_played(&paths, &tag);
    let _ = app.emit("game-started", &game.id);
    let game_id = game.id.clone();
    std::thread::spawn(move || {
        let _ = child.wait();
        app.state::<AppState>().jobs.finish_game(&game_id);
        let _ = app.emit("game-exited", &game_id);
    });
    Ok(())
}

#[tauri::command]
pub fn get_running(state: State<'_, AppState>) -> Vec<String> {
    state.jobs.running()
}

#[tauri::command]
pub async fn open_install_dir(
    state: State<'_, AppState>,
    id: String,
    tag: String,
) -> Result<(), AppError> {
    let game = state.game(&id).await?;
    let paths = GamePaths::new(&state.install_root, &game.id);
    if !registry::is_installed(&paths, &tag)? {
        return Err(AppError::NotInstalled);
    }
    open(&paths.version(&tag)?)
}

#[tauri::command]
pub async fn open_saves_dir(state: State<'_, AppState>, id: String) -> Result<(), AppError> {
    let game = state.game(&id).await?;
    let dir = saves_dir(&game.saves, Platform::current().os, &state.home_dir)
        .filter(|dir| dir.is_dir())
        .ok_or(AppError::NoSaves)?;
    open(&dir)
}

fn open(dir: &Path) -> Result<(), AppError> {
    tauri_plugin_opener::open_path(dir, None::<&str>).map_err(|e| AppError::Io(e.to_string()))
}
