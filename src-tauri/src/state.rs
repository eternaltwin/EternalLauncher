use crate::catalog::load::{LoadedCatalog, load};
use crate::catalog::model::Game;
use crate::error::AppError;
use crate::github::client::fetch_releases;
use crate::github::releases::Release;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, Instant};
use tauri::async_runtime::Mutex as AsyncMutex;

/// Les releases déjà lues restent valables 10 minutes : l'API GitHub limite les requêtes.
const RELEASES_TTL: Duration = Duration::from_secs(10 * 60);

pub struct AppState {
    pub http: reqwest::Client,
    pub cache_dir: PathBuf,
    catalog: AsyncMutex<Option<LoadedCatalog>>,
    releases: Mutex<HashMap<String, (Instant, Vec<Release>)>>,
}

impl AppState {
    pub fn new(cache_dir: PathBuf) -> Result<Self, AppError> {
        Ok(Self {
            http: crate::http::client()?,
            cache_dir,
            catalog: AsyncMutex::new(None),
            releases: Mutex::new(HashMap::new()),
        })
    }

    pub fn cache_dir(&self) -> &Path {
        &self.cache_dir
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

    pub async fn releases(&self, repo: &str, refresh: bool) -> Result<Vec<Release>, AppError> {
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
