use crate::error::AppError;
use crate::version::parse_version;
use semver::Version;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// Runtime demandé par le catalogue (`hashlink>=1.16`). Seul HashLink est connu pour l'instant.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Requirement {
    pub name: String,
    pub min: Option<Version>,
}

const KNOWN: [&str; 1] = ["hashlink"];

impl Requirement {
    pub fn parse(text: &str) -> Option<Self> {
        let (name, min) = match text.split_once(">=") {
            Some((name, min)) => (name.trim(), Some(parse_version(min)?)),
            None => (text.trim(), None),
        };
        KNOWN.contains(&name).then(|| Self {
            name: name.to_string(),
            min,
        })
    }

    /// Le programme du runtime sur cette machine, à la bonne version.
    pub fn locate(&self) -> Result<PathBuf, AppError> {
        let min = self.min.as_ref().map(ToString::to_string);
        let Some(path) = find_program("hl") else {
            return Err(AppError::RuntimeMissing {
                runtime: self.name.clone(),
                min,
            });
        };
        // Une version illisible ne bloque pas : on laisse le jeu essayer.
        if let (Some(required), Some(found)) = (&self.min, program_version(&path))
            && &found < required
        {
            return Err(AppError::RuntimeTooOld {
                runtime: self.name.clone(),
                found: found.to_string(),
                min: required.to_string(),
            });
        }
        Ok(path)
    }
}

/// `hl --version` répond aussitôt ; au-delà, le programme trouvé n'est pas celui attendu.
const VERSION_TIMEOUT: Duration = Duration::from_secs(5);

/// Cherche un programme dans le PATH, puis là où Homebrew l'installe.
fn find_program(name: &str) -> Option<PathBuf> {
    let file = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    let in_path = std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).collect::<Vec<_>>())
        .unwrap_or_default();
    let homebrew = [
        PathBuf::from("/opt/homebrew/bin"),
        PathBuf::from("/usr/local/bin"),
    ];
    find_in(&file, in_path.into_iter().chain(homebrew))
}

/// Seuls les dossiers absolus comptent : une entrée vide ou relative du PATH (`.`, `bin`)
/// désignerait le dossier courant du launcher, où n'importe quel `hl` téléchargé pourrait
/// se trouver.
fn find_in(file: &str, dirs: impl IntoIterator<Item = PathBuf>) -> Option<PathBuf> {
    dirs.into_iter()
        .filter(|dir| dir.is_absolute())
        .map(|dir| dir.join(file))
        .find(|candidate| candidate.is_file())
}

fn program_version(path: &Path) -> Option<Version> {
    let mut child = Command::new(path)
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()?;
    let started = Instant::now();
    while child.try_wait().ok()?.is_none() {
        if started.elapsed() > VERSION_TIMEOUT {
            let _ = child.kill();
            let _ = child.wait();
            return None;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let mut text = String::new();
    child.stdout.take()?.read_to_string(&mut text).ok()?;
    text.push(' ');
    child.stderr.take()?.read_to_string(&mut text).ok()?;
    first_version(&text)
}

/// Premier mot qui ressemble à une version (`1.16.0`) dans la sortie du programme.
fn first_version(text: &str) -> Option<Version> {
    text.split_whitespace()
        .filter(|word| word.contains('.'))
        .find_map(|word| parse_version(word.trim_matches(|c: char| !c.is_ascii_alphanumeric())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lit_l_exigence_du_catalogue() {
        let requirement = Requirement::parse("hashlink>=1.16").unwrap();
        assert_eq!(requirement.name, "hashlink");
        assert_eq!(requirement.min, Some(Version::new(1, 16, 0)));
        assert_eq!(Requirement::parse("hashlink").unwrap().min, None);
    }

    #[test]
    fn refuse_un_runtime_inconnu_ou_illisible() {
        assert!(Requirement::parse("java>=8").is_none());
        assert!(Requirement::parse("hashlink>=abc").is_none());
    }

    #[test]
    fn ignore_les_dossiers_relatifs_du_path() {
        // `cargo test` tourne depuis `src-tauri` : `src/lib.rs` y existe en relatif.
        assert!(Path::new("src").join("lib.rs").is_file());
        assert_eq!(
            find_in("lib.rs", [PathBuf::from("src"), PathBuf::new()]),
            None
        );
        let absolute = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        assert_eq!(
            find_in("lib.rs", [absolute.clone()]),
            Some(absolute.join("lib.rs"))
        );
    }

    #[test]
    fn trouve_la_version_dans_la_sortie_du_programme() {
        assert_eq!(first_version("1.16.0"), Some(Version::new(1, 16, 0)));
        assert_eq!(
            first_version("HashLink v1.15.0 (64 bits)"),
            Some(Version::new(1, 15, 0))
        );
        assert_eq!(first_version("usage: hl <file>"), None);
    }
}
