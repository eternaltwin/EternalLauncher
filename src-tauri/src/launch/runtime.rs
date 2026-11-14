use crate::error::AppError;
use crate::version::parse_version;
use semver::Version;
use std::path::PathBuf;
use std::process::{Command, Stdio};

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
    in_path
        .into_iter()
        .chain(homebrew)
        .map(|dir| dir.join(&file))
        .find(|candidate| candidate.is_file())
}

fn program_version(path: &PathBuf) -> Option<Version> {
    let output = Command::new(path)
        .arg("--version")
        .stdin(Stdio::null())
        .output()
        .ok()?;
    let text = format!(
        "{} {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
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
    fn trouve_la_version_dans_la_sortie_du_programme() {
        assert_eq!(first_version("1.16.0"), Some(Version::new(1, 16, 0)));
        assert_eq!(
            first_version("HashLink v1.15.0 (64 bits)"),
            Some(Version::new(1, 15, 0))
        );
        assert_eq!(first_version("usage: hl <file>"), None);
    }
}
