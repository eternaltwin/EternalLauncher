use crate::error::AppError;
use std::time::Duration;

/// Délai d'une petite requête (catalogue, icône, API GitHub), du début à la dernière donnée.
pub const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Client HTTP partagé : HTTPS seulement, et un User-Agent qui nous identifie (GitHub refuse
/// les requêtes sans). Pas de délai global, qui couperait un gros téléchargement sur une
/// connexion lente : une connexion muette 30 secondes est abandonnée, et chaque petite
/// requête pose son propre délai (`REQUEST_TIMEOUT`).
pub fn client() -> Result<reqwest::Client, AppError> {
    let user_agent = format!(
        "EternalLauncher/{} (+https://github.com/eternaltwin/EternalLauncher)",
        env!("CARGO_PKG_VERSION")
    );
    Ok(reqwest::Client::builder()
        .user_agent(user_agent)
        .https_only(true)
        .connect_timeout(Duration::from_secs(10))
        .read_timeout(Duration::from_secs(30))
        .build()?)
}

/// Télécharge un petit fichier en refusant tout ce qui dépasse `max_bytes`.
pub async fn get_bytes(
    http: &reqwest::Client,
    url: &str,
    max_bytes: usize,
) -> Result<Vec<u8>, AppError> {
    let response = http
        .get(url)
        .timeout(REQUEST_TIMEOUT)
        .send()
        .await?
        .error_for_status()?;
    read_limited(response, max_bytes, url).await
}

/// Lit le corps d'une réponse, sans jamais garder plus de `max_bytes` en mémoire.
pub async fn read_limited(
    mut response: reqwest::Response,
    max_bytes: usize,
    what: &str,
) -> Result<Vec<u8>, AppError> {
    let too_large = || AppError::TooLarge(what.to_string());
    if response
        .content_length()
        .is_some_and(|length| length > max_bytes as u64)
    {
        return Err(too_large());
    }
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        if bytes.len() + chunk.len() > max_bytes {
            return Err(too_large());
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}
