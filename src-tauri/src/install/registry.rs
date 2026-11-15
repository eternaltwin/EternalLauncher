use super::paths::{GamePaths, MARKER, safe_tag};
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Ce qu'on garde d'une version installée, dans son propre dossier.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InstalledVersion {
    pub tag: String,
    pub version: String,
    pub asset: String,
    pub sha256: String,
    pub installed_at: u64,
}

pub fn write_marker(dir: &Path, installed: &InstalledVersion) -> Result<(), AppError> {
    let text = serde_json::to_string_pretty(installed).map_err(|e| AppError::Io(e.to_string()))?;
    std::fs::write(dir.join(MARKER), text)?;
    Ok(())
}

/// Les versions installées d'un jeu : les dossiers qui ont leur fichier témoin.
pub fn list_installed(paths: &GamePaths) -> Vec<InstalledVersion> {
    let Ok(entries) = std::fs::read_dir(paths.dir()) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .filter_map(|entry| {
            let name = entry.file_name().into_string().ok()?;
            safe_tag(&name).ok()?;
            let text = std::fs::read_to_string(entry.path().join(MARKER)).ok()?;
            let installed: InstalledVersion = serde_json::from_str(&text).ok()?;
            (installed.tag == name).then_some(installed)
        })
        .collect()
}

pub fn is_installed(paths: &GamePaths, tag: &str) -> Result<bool, AppError> {
    Ok(paths.version(tag)?.join(MARKER).is_file())
}

/// Supprime une version installée, et seulement elle.
pub fn remove(paths: &GamePaths, tag: &str) -> Result<(), AppError> {
    if !is_installed(paths, tag)? {
        return Err(AppError::NotInstalled);
    }
    std::fs::remove_dir_all(paths.version(tag)?)?;
    Ok(())
}

pub fn last_played(paths: &GamePaths) -> Option<String> {
    let tag = std::fs::read_to_string(paths.last_played()).ok()?;
    safe_tag(tag.trim()).ok().map(str::to_string)
}

pub fn set_last_played(paths: &GamePaths, tag: &str) -> Result<(), AppError> {
    std::fs::write(paths.last_played(), safe_tag(tag)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_root(name: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!("el-registry-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        root
    }

    fn installed(tag: &str) -> InstalledVersion {
        InstalledVersion {
            tag: tag.to_string(),
            version: tag.trim_start_matches('v').to_string(),
            asset: format!("jeu-{tag}.zip"),
            sha256: "00".to_string(),
            installed_at: 1,
        }
    }

    #[test]
    fn ne_liste_que_les_versions_completes() {
        let root = temp_root("liste");
        let paths = GamePaths::new(&root, "jeu");
        let complete = paths.version("v1.0").unwrap();
        std::fs::create_dir_all(&complete).unwrap();
        write_marker(&complete, &installed("v1.0")).unwrap();
        std::fs::create_dir_all(paths.version("v0.9").unwrap()).unwrap();
        std::fs::create_dir_all(paths.temp("v1.1").unwrap()).unwrap();
        assert_eq!(list_installed(&paths), vec![installed("v1.0")]);
        remove(&paths, "v1.0").unwrap();
        assert!(list_installed(&paths).is_empty());
        assert!(matches!(
            remove(&paths, "v0.9"),
            Err(AppError::NotInstalled)
        ));
        std::fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn retient_la_derniere_version_jouee() {
        let root = temp_root("dernier");
        let paths = GamePaths::new(&root, "jeu");
        assert_eq!(last_played(&paths), None);
        std::fs::create_dir_all(paths.dir()).unwrap();
        set_last_played(&paths, "v0.8").unwrap();
        assert_eq!(last_played(&paths).as_deref(), Some("v0.8"));
        std::fs::remove_dir_all(&root).unwrap();
    }
}
