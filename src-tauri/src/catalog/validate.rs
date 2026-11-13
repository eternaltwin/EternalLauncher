use super::model::{Catalog, Game, RawCatalog, Source};
use crate::error::AppError;
use crate::version::parse_version;
use std::collections::HashSet;

pub const SCHEMA_VERSION: u32 = 1;

const SUPPORTED_LANGUAGES: [&str; 5] = ["fr", "en", "es", "de", "pt-BR"];

/// Lit le catalogue en gardant les jeux valides et compris par cette version du launcher.
pub fn parse(text: &str, launcher_version: &str) -> Result<Catalog, AppError> {
    let raw: RawCatalog =
        serde_json::from_str(text).map_err(|e| AppError::InvalidCatalog(e.to_string()))?;
    if raw.schema != SCHEMA_VERSION {
        return Err(AppError::InvalidCatalog(format!(
            "format {} inconnu",
            raw.schema
        )));
    }
    let launcher = parse_version(launcher_version);
    let mut seen = HashSet::new();
    let games = raw
        .games
        .into_iter()
        .filter_map(|value| serde_json::from_value::<Game>(value).ok())
        .filter(|game| check_game(game).is_ok())
        .filter(|game| supported_by(game, launcher.as_ref()))
        .filter(|game| seen.insert(game.id.clone()))
        .collect();
    Ok(Catalog { games })
}

/// Version stricte, utilisée par les tests (donc par la CI) : la moindre entrée invalide
/// du catalogue du dépôt fait échouer.
#[cfg(test)]
pub fn parse_strict(text: &str) -> Result<Catalog, AppError> {
    let raw: RawCatalog =
        serde_json::from_str(text).map_err(|e| AppError::InvalidCatalog(e.to_string()))?;
    let mut games = Vec::new();
    let mut seen = HashSet::new();
    for value in raw.games {
        let game: Game =
            serde_json::from_value(value).map_err(|e| AppError::InvalidCatalog(e.to_string()))?;
        check_game(&game).map_err(AppError::InvalidCatalog)?;
        if !seen.insert(game.id.clone()) {
            return Err(AppError::InvalidCatalog(format!(
                "id en double : {}",
                game.id
            )));
        }
        games.push(game);
    }
    Ok(Catalog { games })
}

fn supported_by(game: &Game, launcher: Option<&semver::Version>) -> bool {
    match (game.min_launcher.as_deref().map(parse_version), launcher) {
        (None, _) => true,
        (Some(Some(min)), Some(launcher)) => launcher >= &min,
        _ => false,
    }
}

fn check_game(game: &Game) -> Result<(), String> {
    if !is_slug(&game.id) {
        return Err(format!("id invalide : {}", game.id));
    }
    for (field, text) in [("name", &game.name), ("description", &game.description)] {
        if text.is_empty() || text.values().any(|value| value.trim().is_empty()) {
            return Err(format!("{}: {field} vide", game.id));
        }
        if let Some(lang) = text
            .keys()
            .find(|lang| !SUPPORTED_LANGUAGES.contains(&lang.as_str()))
        {
            return Err(format!("{}: langue inconnue {lang}", game.id));
        }
    }
    if !is_icon_path(&game.icon) {
        return Err(format!("{}: icône invalide {}", game.id, game.icon));
    }
    let Source::GitHubReleases {
        repo, min_version, ..
    } = &game.source;
    if !is_repo(repo) {
        return Err(format!("{}: dépôt invalide {repo}", game.id));
    }
    if min_version
        .as_deref()
        .is_some_and(|v| parse_version(v).is_none())
    {
        return Err(format!("{}: minVersion illisible", game.id));
    }
    if game.platforms.is_empty() {
        return Err(format!("{}: aucune plateforme", game.id));
    }
    for (key, spec) in &game.platforms {
        if !is_platform_key(key) {
            return Err(format!("{}: plateforme invalide {key}", game.id));
        }
        if !spec.asset.contains("{tag}") && !spec.asset.contains("{version}") {
            return Err(format!(
                "{}: l'asset de {key} doit contenir {{tag}} ou {{version}}",
                game.id
            ));
        }
    }
    Ok(())
}

fn is_slug(text: &str) -> bool {
    let mut chars = text.chars();
    chars
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

fn is_repo(text: &str) -> bool {
    let valid = |part: &str, extra: &[char]| {
        !part.is_empty()
            && part != "."
            && part != ".."
            && part
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '-' || extra.contains(&c))
    };
    match text.split_once('/') {
        Some((owner, name)) => valid(owner, &[]) && valid(name, &['.', '_']),
        None => false,
    }
}

fn is_icon_path(path: &str) -> bool {
    let Some(file) = path.strip_prefix("icons/") else {
        return false;
    };
    match file.rsplit_once('.') {
        Some((stem, ext)) => is_slug(stem) && ["png", "jpg", "webp", "svg"].contains(&ext),
        None => false,
    }
}

fn is_platform_key(key: &str) -> bool {
    match key.split_once('-') {
        Some((os, arch)) => {
            ["windows", "linux", "macos"].contains(&os)
                && ["x86_64", "aarch64", "*"].contains(&arch)
        }
        None => false,
    }
}

#[cfg(test)]
#[path = "validate_tests.rs"]
mod tests;
