use super::load::{EMBEDDED, REMOTE_BASE};
use super::model::Game;
use crate::error::AppError;
use crate::http::get_bytes;
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use std::path::Path;

const MAX_ICON_BYTES: usize = 512 * 1024;

/// L'icône d'un jeu en URL `data:` : la page n'a ainsi jamais à charger d'image distante.
/// Ordre : copie livrée avec le launcher, cache local, puis le dépôt.
pub async fn icon_data_url(
    http: &reqwest::Client,
    cache_dir: &Path,
    game: &Game,
) -> Result<String, AppError> {
    let mime = mime_type(&game.icon);
    if let Some(file) = EMBEDDED.get_file(&game.icon) {
        return Ok(data_url(mime, file.contents()));
    }
    // `game.icon` est validé (`icons/<slug>.<ext>`) : il ne peut pas sortir du cache.
    let cached = cache_dir.join(&game.icon);
    if let Ok(bytes) = std::fs::read(&cached) {
        return Ok(data_url(mime, &bytes));
    }
    let bytes = get_bytes(http, &format!("{REMOTE_BASE}{}", game.icon), MAX_ICON_BYTES).await?;
    if let Some(parent) = cached.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&cached, &bytes)?;
    Ok(data_url(mime, &bytes))
}

fn mime_type(path: &str) -> &'static str {
    match path.rsplit_once('.').map(|(_, ext)| ext) {
        Some("jpg") => "image/jpeg",
        Some("webp") => "image/webp",
        Some("svg") => "image/svg+xml",
        _ => "image/png",
    }
}

fn data_url(mime: &str, bytes: &[u8]) -> String {
    format!("data:{mime};base64,{}", STANDARD.encode(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deduit_le_type_de_l_extension() {
        assert_eq!(mime_type("icons/a.svg"), "image/svg+xml");
        assert_eq!(mime_type("icons/a.jpg"), "image/jpeg");
        assert_eq!(mime_type("icons/a.png"), "image/png");
    }

    #[test]
    fn encode_en_base64() {
        assert_eq!(data_url("image/png", b"abc"), "data:image/png;base64,YWJj");
    }
}
