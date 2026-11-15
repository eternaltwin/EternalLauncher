use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Dossier des sauvegardes d'un jeu sur cette machine, d'après le catalogue :
/// `%APPDATA%/…` sous Windows, `~/…` ailleurs. Les chemins sont validés au chargement.
pub fn saves_dir(saves: &BTreeMap<String, String>, os: &str, home: &Path) -> Option<PathBuf> {
    let path = saves.get(os)?;
    if let Some(rest) = path.strip_prefix("%APPDATA%/") {
        let appdata = std::env::var_os("APPDATA")?;
        return Some(PathBuf::from(appdata).join(rest));
    }
    path.strip_prefix("~/").map(|rest| home.join(rest))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_du_dossier_personnel() {
        let saves = BTreeMap::from([("linux".to_string(), "~/Galaxy55".to_string())]);
        assert_eq!(
            saves_dir(&saves, "linux", Path::new("/home/joueur")),
            Some(Path::new("/home/joueur").join("Galaxy55"))
        );
        assert_eq!(saves_dir(&saves, "macos", Path::new("/Users/j")), None);
    }
}
