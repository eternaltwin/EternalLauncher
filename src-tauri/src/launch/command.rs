use super::runtime::Requirement;
use crate::catalog::launch_rules::is_relative_path;
use crate::catalog::model::PlatformSpec;
use crate::error::AppError;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};

/// Ce qu'il faut exécuter pour lancer une version : jamais un shell, jamais un script du zip.
#[derive(Debug, PartialEq, Eq)]
pub struct LaunchCommand {
    pub program: PathBuf,
    pub args: Vec<String>,
    pub cwd: PathBuf,
}

impl LaunchCommand {
    /// Programme livré dans le dossier de la version, ou runtime de la machine.
    /// `variant` choisit d'autres arguments (DirectX…).
    pub fn build(
        spec: &PlatformSpec,
        version_dir: &Path,
        variant: Option<&str>,
    ) -> Result<Self, AppError> {
        let args = match variant {
            None => spec.args.clone(),
            Some(id) => spec
                .variants
                .iter()
                .find(|v| v.id == id)
                .map(|v| v.args.clone())
                .ok_or_else(|| AppError::Launch(format!("variante inconnue : {id}")))?,
        };
        let program = match (&spec.program, &spec.runtime) {
            (Some(program), _) if is_relative_path(program) => version_dir.join(program),
            (None, Some(runtime)) => Requirement::parse(runtime)
                .ok_or_else(|| AppError::Launch(format!("runtime inconnu : {runtime}")))?
                .locate()?,
            _ => return Err(AppError::Launch("programme à lancer inconnu".to_string())),
        };
        if !program.is_file() {
            return Err(AppError::Launch(format!(
                "{} introuvable",
                program.display()
            )));
        }
        Ok(Self {
            program,
            args,
            cwd: version_dir.to_path_buf(),
        })
    }

    /// Démarre le jeu depuis le dossier de sa version : il y lit ses ressources.
    pub fn spawn(&self) -> Result<Child, AppError> {
        Command::new(&self.program)
            .args(&self.args)
            .current_dir(&self.cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| AppError::Launch(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::model::Variant;

    fn version_dir() -> PathBuf {
        let dir = std::env::temp_dir().join(format!("el-launch-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("galaxy55.exe"), b"").unwrap();
        dir
    }

    fn windows_spec() -> PlatformSpec {
        PlatformSpec {
            asset: "{tag}.zip".to_string(),
            program: Some("galaxy55.exe".to_string()),
            args: vec!["galaxy55.hl".to_string()],
            variants: vec![Variant {
                id: "directx".to_string(),
                args: vec!["galaxy55_dx.hl".to_string()],
            }],
            ..PlatformSpec::default()
        }
    }

    #[test]
    fn lance_le_programme_du_dossier_avec_ses_arguments() {
        let dir = version_dir();
        let command = LaunchCommand::build(&windows_spec(), &dir, None).unwrap();
        assert_eq!(command.program, dir.join("galaxy55.exe"));
        assert_eq!(command.args, ["galaxy55.hl"]);
        assert_eq!(command.cwd, dir);
        let directx = LaunchCommand::build(&windows_spec(), &dir, Some("directx")).unwrap();
        assert_eq!(directx.args, ["galaxy55_dx.hl"]);
    }

    #[test]
    fn refuse_une_variante_ou_un_programme_absents() {
        let dir = version_dir();
        assert!(LaunchCommand::build(&windows_spec(), &dir, Some("vulkan")).is_err());
        let mut missing = windows_spec();
        missing.program = Some("absent.exe".to_string());
        assert!(LaunchCommand::build(&missing, &dir, None).is_err());
        let mut escape = windows_spec();
        escape.program = Some("../galaxy55.exe".to_string());
        assert!(LaunchCommand::build(&escape, &dir, None).is_err());
    }
}
