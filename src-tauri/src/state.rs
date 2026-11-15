use crate::catalog::load::{LoadedCatalog, load};
use crate::catalog::model::{Game, PlatformSpec, Source};
use crate::error::AppError;
use crate::github::client::fetch_releases;
use crate::github::releases::{GameVersion, Release, VersionFilter, game_versions};
use crate::jobs::Jobs;
use crate::platform::Platform;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::async_runtime::Mutex as AsyncMutex;

/// Les releases déjà lues restent valables 10 minutes : l'API GitHub limite les requêtes.
const RELEASES_TTL: Duration = Duration::from_secs(10 * 60);

pub struct AppState {
    pub http: reqwest::Client,
    pub cache_dir: PathBuf,
    /// Dossier des jeux installés : `<racine>/<id>/<tag>/`.
    pub install_root: PathBuf,
    pub home_dir: PathBuf,
    pub jobs: Jobs,
    catalog: AsyncMutex<Option<LoadedCatalog>>,
    releases: Mutex<HashMap<String, (Instant, Vec<Release>)>>,
}

impl AppState {
    pub fn new(
        cache_dir: PathBuf,
        install_root: PathBuf,
        home_dir: PathBuf,
    ) -> Result<Self, AppError> {
        Ok(Self {
            http: crate::http::client()?,
            cache_dir,
            install_root,
            home_dir,
            jobs: Jobs::default(),
            catalog: AsyncMutex::new(None),
            releases: Mutex::new(HashMap::new()),
        })
    }

    /// Le catalogue, chargé une fois par session ; `refresh` le relit.
    pub async fn catalog(&self, refresh: bool) -> LoadedCatalog {
        let mut current = self.catalog.lock().await;
        match current.as_ref() {
            Some(loaded) if !refresh => loaded.clone(),
            _ => {
                let loaded = load(&self.http, &self.cache_dir).await;
                *current = Some(loaded.clone());
                loaded
            }
        }
    }

    pub async fn game(&self, id: &str) -> Result<Game, AppError> {
        self.catalog(false)
            .await
            .catalog
            .games
            .into_iter()
            .find(|game| game.id == id)
            .ok_or_else(|| AppError::UnknownGame(id.to_string()))
    }

    /// Le jeu et sa plateforme pour cette machine.
    pub async fn game_for_platform(&self, id: &str) -> Result<(Game, PlatformSpec), AppError> {
        let game = self.game(id).await?;
        let spec = Platform::current()
            .find(&game.platforms)
            .cloned()
            .ok_or(AppError::Unavailable)?;
        Ok((game, spec))
    }

    /// Les versions du jeu proposées pour cette machine, les plus récentes d'abord.
    pub async fn versions(
        &self,
        game: &Game,
        spec: &PlatformSpec,
        refresh: bool,
    ) -> Result<Vec<GameVersion>, AppError> {
        let Source::GitHubReleases {
            repo,
            min_version,
            prereleases,
        } = &game.source;
        let releases = self.releases(repo, refresh).await?;
        let filter = VersionFilter {
            min_version: min_version.as_deref(),
            prereleases: *prereleases,
        };
        Ok(game_versions(&releases, &filter, spec))
    }

    async fn releases(&self, repo: &str, refresh: bool) -> Result<Vec<Release>, AppError> {
        if !refresh && let Some(cached) = self.cached_releases(repo) {
            return Ok(cached);
        }
        let releases = fetch_releases(&self.http, repo).await?;
        if let Ok(mut cache) = self.releases.lock() {
            cache.insert(repo.to_string(), (Instant::now(), releases.clone()));
        }
        Ok(releases)
    }

    fn cached_releases(&self, repo: &str) -> Option<Vec<Release>> {
        let cache = self.releases.lock().ok()?;
        let (at, releases) = cache.get(repo)?;
        (at.elapsed() < RELEASES_TTL).then(|| releases.clone())
    }
}
