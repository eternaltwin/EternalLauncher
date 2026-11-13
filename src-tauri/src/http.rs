use crate::error::AppError;
use std::time::Duration;

/// Client HTTP partagé : HTTPS seulement, délais bornés, et un User-Agent qui nous identifie
/// (GitHub refuse les requêtes sans).
pub fn client() -> Result<reqwest::Client, AppError> {
    let user_agent = format!(
        "EternalLauncher/{} (+https://github.com/eternaltwin/EternalLauncher)",
        env!("CARGO_PKG_VERSION")
    );
    Ok(reqwest::Client::builder()
        .user_agent(user_agent)
        .https_only(true)
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(30))
        .build()?)
}

/// Télécharge un petit fichier en refusant tout ce qui dépasse `max_bytes`.
pub async fn get_bytes(
    http: &reqwest::Client,
    url: &str,
    max_bytes: usize,
) -> Result<Vec<u8>, AppError> {
    let mut response = http.get(url).send().await?.error_for_status()?;
    let mut bytes = Vec::new();
    while let Some(chunk) = response.chunk().await? {
        if bytes.len() + chunk.len() > max_bytes {
            return Err(AppError::TooLarge(url.to_string()));
        }
        bytes.extend_from_slice(&chunk);
    }
    Ok(bytes)
}
