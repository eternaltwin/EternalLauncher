use super::releases::Release;
use crate::error::AppError;
use crate::http::{REQUEST_TIMEOUT, read_limited};
use reqwest::StatusCode;
use reqwest::header::ACCEPT;

const API: &str = "https://api.github.com";

/// 100 releases aux notes les plus longues que GitHub accepte (125 000 caractères) tiennent
/// dans cette limite ; au-delà, la réponse n'est pas celle de GitHub.
const MAX_RELEASES_BYTES: usize = 20 * 1024 * 1024;

/// Releases d'un dépôt, les plus récentes d'abord. Sans compte GitHub, l'API accepte
/// 60 requêtes par heure et par adresse IP : l'appelant garde les réponses en cache.
pub async fn fetch_releases(http: &reqwest::Client, repo: &str) -> Result<Vec<Release>, AppError> {
    // `repo` vient du catalogue validé (`propriétaire/nom`, caractères sûrs).
    let url = format!("{API}/repos/{repo}/releases?per_page=100");
    let response = http
        .get(&url)
        .header(ACCEPT, "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .timeout(REQUEST_TIMEOUT)
        .send()
        .await?;
    let status = response.status();
    if status.is_success() {
        let bytes = read_limited(response, MAX_RELEASES_BYTES, &url).await?;
        return serde_json::from_slice(&bytes).map_err(|_| AppError::GitHub(status.as_u16()));
    }
    let remaining = header(&response, "x-ratelimit-remaining");
    if status == StatusCode::TOO_MANY_REQUESTS
        || (status == StatusCode::FORBIDDEN && remaining.as_deref() == Some("0"))
    {
        let reset = header(&response, "x-ratelimit-reset").and_then(|v| v.parse().ok());
        return Err(AppError::RateLimited { reset });
    }
    Err(AppError::GitHub(status.as_u16()))
}

fn header(response: &reqwest::Response, name: &str) -> Option<String> {
    response
        .headers()
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_string)
}
