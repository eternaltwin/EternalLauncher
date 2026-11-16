use crate::catalog::model::Source;
use crate::error::AppError;
use crate::install::download::{Expected, download};
use crate::install::paths::GamePaths;
use crate::install::place::{place, remove};
use crate::install::registry::{self, InstalledVersion};
use crate::jobs::install_key;
use crate::launch::saves::saves_dir;
use crate::platform::Platform;
use crate::state::AppState;
use serde::Serialize;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter, State};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledState {
    pub versions: Vec<InstalledVersion>,
    pub last_played: Option<String>,
    pub running: bool,
    pub has_saves: bool,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct InstallProgress<'a> {
    game_id: &'a str,
    tag: &'a str,
    phase: &'static str,
    received: u64,
    total: u64,
}

#[tauri::command]
pub async fn get_installed(
    state: State<'_, AppState>,
    id: String,
) -> Result<InstalledState, AppError> {
    let game = state.game(&id).await?;
    let paths = GamePaths::new(&state.install_root, &game.id);
    let saves = saves_dir(&game.saves, Platform::current().os, &state.home_dir);
    Ok(InstalledState {
        versions: registry::list_installed(&paths),
        last_played: registry::last_played(&paths),
        running: state.jobs.is_running(&game.id),
        has_saves: saves.is_some_and(|dir| dir.is_dir()),
    })
}

/// Télécharge, vérifie et installe une version. Rien n'est gardé si une étape échoue.
#[tauri::command]
pub async fn install_version(
    app: AppHandle,
    state: State<'_, AppState>,
    id: String,
    tag: String,
) -> Result<InstalledVersion, AppError> {
    let (game, spec) = state.game_for_platform(&id).await?;
    let paths = GamePaths::new(&state.install_root, &game.id);
    // Le verrou avant toute vérification : deux demandes pour la même version ne peuvent pas
    // la trouver toutes les deux « pas installée ».
    let guard = state.jobs.start_install(install_key(&game.id, &tag))?;
    if registry::is_installed(&paths, &tag)? {
        return Err(AppError::Busy);
    }
    let version = state
        .versions(&game, &spec, false)
        .await?
        .into_iter()
        .find(|v| v.tag == tag)
        .ok_or_else(|| AppError::UnknownVersion(tag.clone()))?;
    let sha256 = version
        .asset
        .sha256
        .clone()
        .ok_or(AppError::MissingDigest)?;
    let Source::GitHubReleases { repo, .. } = &game.source;
    // Seulement un asset des releases du dépôt du catalogue.
    let prefix = format!("https://github.com/{repo}/releases/download/").to_ascii_lowercase();
    if !version
        .asset
        .download_url
        .to_ascii_lowercase()
        .starts_with(&prefix)
    {
        return Err(AppError::UnknownVersion(tag));
    }

    let partial = paths.download(&tag)?;
    let total = version.asset.size;
    let emit = |phase, received| {
        let progress = InstallProgress {
            game_id: &game.id,
            tag: &tag,
            phase,
            received,
            total,
        };
        let _ = app.emit("install-progress", progress);
    };
    let mut last_percent = u64::MAX;
    let expected = Expected {
        url: &version.asset.download_url,
        size: total,
        sha256: &sha256,
    };
    download(
        &state.http,
        &expected,
        &partial,
        guard.cancel_flag(),
        |received| {
            let percent = received * 100 / total.max(1);
            if percent != last_percent {
                last_percent = percent;
                emit("download", received);
            }
        },
    )
    .await?;

    emit("extract", total);
    let installed = InstalledVersion {
        tag: tag.clone(),
        version: version.version.clone(),
        asset: version.asset.name.clone(),
        sha256,
        installed_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |d| d.as_secs()),
    };
    let result = {
        let (paths, partial, installed) = (paths.clone(), partial.clone(), installed.clone());
        tauri::async_runtime::spawn_blocking(move || place(&paths, &partial, &installed))
            .await
            .map_err(|e| AppError::Io(e.to_string()))?
    };
    let _ = std::fs::remove_file(&partial);
    result?;
    emit("done", total);
    Ok(installed)
}

#[tauri::command]
pub fn cancel_install(state: State<'_, AppState>, id: String, tag: String) {
    state.jobs.cancel_install(&install_key(&id, &tag));
}

#[tauri::command]
pub async fn uninstall_version(
    state: State<'_, AppState>,
    id: String,
    tag: String,
) -> Result<(), AppError> {
    let game = state.game(&id).await?;
    if state.jobs.is_running(&game.id) {
        return Err(AppError::AlreadyRunning);
    }
    remove(&GamePaths::new(&state.install_root, &game.id), &tag)
}
