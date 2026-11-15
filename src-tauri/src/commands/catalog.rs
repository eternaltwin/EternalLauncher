use crate::catalog::icon::icon_data_url;
use crate::catalog::load::Origin;
use crate::catalog::model::{Game, GameKind, Localized, Source};
use crate::error::AppError;
use crate::github::releases::GameVersion;
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
    /// Autres façons de lancer le jeu sur ce système (`directx`…).
    pub variants: Vec<String>,
}

impl GameSummary {
    fn new(game: &Game, platform: &Platform) -> Self {
        let Source::GitHubReleases { repo, .. } = &game.source;
        let spec = platform.find(&game.platforms);
        Self {
            id: game.id.clone(),
            kind: game.kind,
            name: game.name.clone(),
            description: game.description.clone(),
            repo: repo.clone(),
            repo_url: format!("https://github.com/{repo}"),
            available: spec.is_some(),
            variants: spec
                .map(|spec| spec.variants.iter().map(|v| v.id.clone()).collect())
                .unwrap_or_default(),
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
    match state.game_for_platform(&id).await {
        Ok((game, spec)) => state.versions(&game, &spec, refresh).await,
        Err(AppError::Unavailable) => Ok(Vec::new()),
        Err(error) => Err(error),
    }
}

#[tauri::command]
pub async fn get_game_icon(state: State<'_, AppState>, id: String) -> Result<String, AppError> {
    let game = state.game(&id).await?;
    icon_data_url(&state.http, &state.cache_dir, &game).await
}
