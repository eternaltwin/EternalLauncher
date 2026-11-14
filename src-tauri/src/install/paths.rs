use crate::error::AppError;
use std::path::{Path, PathBuf};

/// Fichier écrit en dernier dans le dossier d'une version : sans lui, la version n'est pas
/// installée (téléchargement ou extraction interrompus).
pub const MARKER: &str = ".eternallauncher.json";

/// Un tag GitHub sert de nom de dossier : lettres, chiffres, `.`, `_` et `-` seulement.
pub fn safe_tag(tag: &str) -> Result<&str, AppError> {
    let valid = !tag.is_empty()
        && tag.len() <= 64
        && !tag.starts_with('.')
        && tag
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '-'));
    if valid {
        Ok(tag)
    } else {
        Err(AppError::UnknownVersion(tag.to_string()))
    }
}

/// Dossiers d'un jeu : `<racine>/<id>/<tag>/` pour chaque version installée, et des dossiers
/// cachés (`.downloads`, `.tmp-<tag>`) pour le travail en cours.
pub struct GamePaths {
    dir: PathBuf,
}

impl GamePaths {
    /// `game_id` vient du catalogue validé (minuscules, chiffres, tirets).
    pub fn new(install_root: &Path, game_id: &str) -> Self {
        Self {
            dir: install_root.join(game_id),
        }
    }

    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn version(&self, tag: &str) -> Result<PathBuf, AppError> {
        Ok(self.dir.join(safe_tag(tag)?))
    }

    pub fn temp(&self, tag: &str) -> Result<PathBuf, AppError> {
        Ok(self.dir.join(format!(".tmp-{}", safe_tag(tag)?)))
    }

    pub fn download(&self, tag: &str) -> Result<PathBuf, AppError> {
        Ok(self
            .dir
            .join(".downloads")
            .join(format!("{}.zip.partial", safe_tag(tag)?)))
    }

    pub fn last_played(&self) -> PathBuf {
        self.dir.join(".last-played")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepte_les_tags_ordinaires() {
        assert!(safe_tag("v0.8.1").is_ok());
        assert!(safe_tag("1.0-beta_2").is_ok());
    }

    #[test]
    fn refuse_les_tags_qui_sortiraient_du_dossier() {
        for tag in ["", "..", ".cache", "a/b", "a\\b", "v1 2", "c:x"] {
            assert!(safe_tag(tag).is_err(), "{tag}");
        }
    }

    #[test]
    fn range_chaque_version_dans_le_dossier_du_jeu() {
        let paths = GamePaths::new(Path::new("racine"), "galaxy55");
        assert_eq!(
            paths.version("v0.8").unwrap(),
            Path::new("racine").join("galaxy55").join("v0.8")
        );
        assert!(paths.temp("../x").is_err());
    }
}
