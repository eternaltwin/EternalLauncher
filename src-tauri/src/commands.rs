use crate::catalog::icon::icon_data_url;
use crate::catalog::load::Origin;
use crate::catalog::model::{Game, GameKind, Localized, Source};
use crate::error::AppError;
use crate::github::releases::{GameVersion, VersionFilter, game_versions};
use crate::platform::Platform;
use crate::state::AppState;
use serde::Serialize;
use tauri::State;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogView {
    pub origin: Origin,
    pub platform: String,
    pub games: Vec<GameSummary>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameSummary {
    pub id: String,
    pub kind: GameKind,
    pub name: Localized,
    pub description: Localized,
    pub repo: String,
    pub repo_url: String,
    /// Faux quand le jeu ne propose rien pour ce système.
    pub available: bool,
}

impl GameSummary {
    fn new(game: &Game, platform: &Platform) -> Self {
        let Source::GitHubReleases { repo, .. } = &game.source;
        Self {
            id: game.id.clone(),
            kind: game.kind,
            name: game.name.clone(),
            description: game.description.clone(),
            repo: repo.clone(),
            repo_url: format!("https://github.com/{repo}"),
            available: platform.find(&game.platforms).is_some(),
        }
    }
}

#[tauri::command]
pub async fn get_catalog(
    state: State<'_, AppState>,
    refresh: bool,
) -> Result<CatalogView, AppError> {
    let platform = Platform::current();
    let loaded = state.catalog(refresh).await;
    Ok(CatalogView {
        origin: loaded.origin,
        platform: format!("{}-{}", platform.os, platform.arch),
        games: loaded
            .catalog
            .games
            .iter()
            .map(|game| GameSummary::new(game, &platform))
            .collect(),
    })
}

#[tauri::command]
pub async fn get_game_versions(
    state: State<'_, AppState>,
    id: String,
    refresh: bool,
) -> Result<Vec<GameVersion>, AppError> {
    let game = state.game(&id).await?;
    let Some(spec) = Platform::current().find(&game.platforms) else {
        return Ok(Vec::new());
    };
    let Source::GitHubReleases {
        repo,
        min_version,
        prereleases,
    } = &game.source;
    let releases = state.releases(repo, refresh).await?;
    let filter = VersionFilter {
        min_version: min_version.as_deref(),
        prereleases: *prereleases,
    };
    Ok(game_versions(&releases, &filter, spec))
}

#[tauri::command]
pub async fn get_game_icon(state: State<'_, AppState>, id: String) -> Result<String, AppError> {
    let game = state.game(&id).await?;
    icon_data_url(&state.http, state.cache_dir(), &game).await
}
