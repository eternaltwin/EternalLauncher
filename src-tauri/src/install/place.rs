use super::extract::extract;
use super::paths::GamePaths;
use super::registry::{self, InstalledVersion};
use crate::error::AppError;
use std::path::Path;

/// Met en place une version téléchargée : extraction dans `.tmp-<tag>`, fichier témoin, puis
/// renommage en `<tag>`. Rien n'est gardé si une étape échoue.
pub fn place(
    paths: &GamePaths,
    archive: &Path,
    installed: &InstalledVersion,
) -> Result<(), AppError> {
    let temp = paths.temp(&installed.tag)?;
    let _ = std::fs::remove_dir_all(&temp);
    let result = extract(archive, &temp)
        .and_then(|()| registry::write_marker(&temp, installed))
        .and_then(|()| promote(paths, &installed.tag, &temp));
    if result.is_err() {
        let _ = std::fs::remove_dir_all(&temp);
    }
    result
}

/// `temp` devient le dossier de la version. Un dossier déjà là sans fichier témoin est le
/// reste d'une suppression interrompue : il est écarté d'abord, sinon le renommage échouerait.
fn promote(paths: &GamePaths, tag: &str, temp: &Path) -> Result<(), AppError> {
    if registry::is_installed(paths, tag)? {
        return Err(AppError::Busy);
    }
    let target = paths.version(tag)?;
    if std::fs::symlink_metadata(&target).is_ok() {
        discard(paths, tag)?;
    }
    std::fs::rename(temp, &target)?;
    Ok(())
}

/// Supprime une version installée, et seulement elle.
pub fn remove(paths: &GamePaths, tag: &str) -> Result<(), AppError> {
    if !registry::is_installed(paths, tag)? {
        return Err(AppError::NotInstalled);
    }
    discard(paths, tag)
}

/// Retire le dossier d'une version d'un seul coup : renommé en `.trash-<tag>`, ce qui échoue
/// sans rien toucher si le jeu tourne encore ou qu'un fichier est ouvert, puis effacé. Un reste
/// de `.trash-<tag>` est invisible et sera effacé à la prochaine suppression.
fn discard(paths: &GamePaths, tag: &str) -> Result<(), AppError> {
    let trash = paths.trash(tag)?;
    let _ = std::fs::remove_dir_all(&trash);
    std::fs::rename(paths.version(tag)?, &trash)?;
    let _ = std::fs::remove_dir_all(&trash);
    Ok(())
}

#[cfg(test)]
#[path = "place_tests.rs"]
mod tests;
