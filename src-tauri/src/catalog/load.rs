use super::model::Catalog;
use super::validate::parse;
use crate::error::AppError;
use crate::http::get_bytes;
use include_dir::{Dir, include_dir};
use serde::Serialize;
use std::path::Path;

/// Le catalogue de la branche `main` : on peut ajouter un jeu sans publier de nouveau launcher.
pub const REMOTE_BASE: &str =
    "https://raw.githubusercontent.com/eternaltwin/EternalLauncher/main/catalog/";

const MAX_CATALOG_BYTES: usize = 1024 * 1024;
const CACHE_FILE: &str = "catalog.json";

/// Copie du catalogue livrée avec le launcher, pour fonctionner hors ligne.
pub static EMBEDDED: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/../catalog");

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Origin {
    Remote,
    Cache,
    Embedded,
}

#[derive(Clone, Debug)]
pub struct LoadedCatalog {
    pub catalog: Catalog,
    pub origin: Origin,
}

/// Catalogue distant, sinon la dernière copie reçue, sinon celle livrée avec le launcher.
pub async fn load(http: &reqwest::Client, cache_dir: &Path) -> LoadedCatalog {
    let version = env!("CARGO_PKG_VERSION");
    let url = format!("{REMOTE_BASE}games.json");
    if let Ok(bytes) = get_bytes(http, &url, MAX_CATALOG_BYTES).await
        && let Ok(text) = String::from_utf8(bytes)
        && let Ok(catalog) = parse(&text, version)
    {
        // Le cache ne sert qu'hors ligne : le perdre n'empêche rien.
        let _ = write_cache(cache_dir, &text);
        return LoadedCatalog {
            catalog,
            origin: Origin::Remote,
        };
    }
    if let Some(catalog) = read_cache(cache_dir).and_then(|text| parse(&text, version).ok()) {
        return LoadedCatalog {
            catalog,
            origin: Origin::Cache,
        };
    }
    LoadedCatalog {
        catalog: embedded(version).unwrap_or(Catalog { games: Vec::new() }),
        origin: Origin::Embedded,
    }
}

fn embedded(version: &str) -> Result<Catalog, AppError> {
    let text = EMBEDDED
        .get_file("games.json")
        .and_then(|file| file.contents_utf8())
        .ok_or_else(|| AppError::InvalidCatalog("catalogue embarqué absent".to_string()))?;
    parse(text, version)
}

fn read_cache(cache_dir: &Path) -> Option<String> {
    std::fs::read_to_string(cache_dir.join(CACHE_FILE)).ok()
}

fn write_cache(cache_dir: &Path, text: &str) -> Result<(), AppError> {
    std::fs::create_dir_all(cache_dir)?;
    std::fs::write(cache_dir.join(CACHE_FILE), text)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn le_catalogue_embarque_contient_galaxy55() {
        let catalog = embedded("0.1.0").unwrap();
        assert!(catalog.games.iter().any(|game| game.id == "galaxy55"));
    }

    #[test]
    fn le_cache_est_relu() {
        let dir =
            std::env::temp_dir().join(format!("eternal-launcher-test-{}", std::process::id()));
        write_cache(&dir, "{}").unwrap();
        assert_eq!(read_cache(&dir).as_deref(), Some("{}"));
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
