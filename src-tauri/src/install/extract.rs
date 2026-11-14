use crate::error::AppError;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

/// Bornes contre les archives piégées (« zip bombs ») : au plus 4 Go décompressés
/// et 20 000 fichiers.
const MAX_TOTAL_BYTES: u64 = 4 * 1024 * 1024 * 1024;
const MAX_ENTRIES: usize = 20_000;

/// Décompresse `zip_path` dans `dest`. Refuse toute l'archive au premier chemin qui
/// sortirait du dossier (absolu, `..`), au premier lien symbolique, ou si elle est trop grosse.
pub fn extract(zip_path: &Path, dest: &Path) -> Result<(), AppError> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file).map_err(bad)?;
    if archive.len() > MAX_ENTRIES {
        return Err(AppError::BadArchive("trop de fichiers".to_string()));
    }
    std::fs::create_dir_all(dest)?;
    let mut total: u64 = 0;
    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).map_err(bad)?;
        let name = entry.name().map(|n| n.into_owned()).unwrap_or_default();
        let Some(relative) = entry.enclosed_name() else {
            return Err(AppError::BadArchive(format!("chemin interdit : {name}")));
        };
        if entry.is_symlink() {
            return Err(AppError::BadArchive(format!("lien symbolique : {name}")));
        }
        let target = dest.join(relative);
        if entry.is_dir() {
            std::fs::create_dir_all(&target)?;
            continue;
        }
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut out = std::fs::File::create(&target)?;
        let budget = MAX_TOTAL_BYTES - total + 1;
        total += std::io::copy(&mut (&mut entry).take(budget), &mut out)?;
        if total > MAX_TOTAL_BYTES {
            return Err(AppError::BadArchive("archive trop grosse".to_string()));
        }
        #[cfg(unix)]
        if let Some(mode) = entry.unix_mode() {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&target, std::fs::Permissions::from_mode(mode & 0o755))?;
        }
    }
    Ok(())
}

fn bad(error: zip::result::ZipError) -> AppError {
    AppError::BadArchive(error.to_string())
}

#[cfg(test)]
#[path = "extract_tests.rs"]
mod tests;
