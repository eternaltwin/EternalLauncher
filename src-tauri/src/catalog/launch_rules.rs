use super::model::PlatformSpec;
use crate::launch::runtime::Requirement;
use std::collections::BTreeMap;
use std::path::{Component, Path};

/// Une plateforme se lance soit par un programme livré dans le zip, soit par un runtime
/// installé sur la machine, jamais les deux.
pub fn check_launch(key: &str, spec: &PlatformSpec) -> Result<(), String> {
    match (&spec.program, &spec.runtime) {
        (Some(program), None) if is_relative_path(program) => {}
        (Some(program), None) => return Err(format!("{key}: programme invalide {program}")),
        (None, Some(runtime)) if Requirement::parse(runtime).is_some() => {}
        (None, Some(runtime)) => return Err(format!("{key}: runtime inconnu {runtime}")),
        _ => return Err(format!("{key}: il faut soit `program`, soit `runtime`")),
    }
    let mut ids = Vec::new();
    for variant in &spec.variants {
        if !is_slug(&variant.id) || ids.contains(&variant.id) {
            return Err(format!("{key}: variante invalide {}", variant.id));
        }
        ids.push(variant.id.clone());
    }
    Ok(())
}

/// Les sauvegardes sont dans le profil du joueur : `%APPDATA%/…` ou `~/…`.
pub fn check_saves(saves: &BTreeMap<String, String>) -> Result<(), String> {
    for (os, path) in saves {
        if !["windows", "linux", "macos"].contains(&os.as_str()) {
            return Err(format!("sauvegardes : système inconnu {os}"));
        }
        let rest = path
            .strip_prefix("%APPDATA%/")
            .or_else(|| path.strip_prefix("~/"));
        if !rest.is_some_and(is_relative_path) {
            return Err(format!("sauvegardes : dossier invalide {path}"));
        }
    }
    Ok(())
}

/// Chemin relatif simple : pas de racine, pas de `..`, pas de `\` ni de `:`.
pub fn is_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['\\', ':'])
        && Path::new(path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn is_slug(text: &str) -> bool {
    !text.is_empty()
        && text
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::catalog::model::Variant;

    fn spec(program: Option<&str>, runtime: Option<&str>) -> PlatformSpec {
        PlatformSpec {
            asset: "{tag}.zip".to_string(),
            program: program.map(str::to_string),
            runtime: runtime.map(str::to_string),
            ..PlatformSpec::default()
        }
    }

    #[test]
    fn accepte_un_programme_ou_un_runtime() {
        assert!(check_launch("windows-x86_64", &spec(Some("galaxy55.exe"), None)).is_ok());
        assert!(check_launch("linux-*", &spec(None, Some("hashlink>=1.16"))).is_ok());
    }

    #[test]
    fn refuse_les_programmes_hors_du_dossier() {
        for program in ["../x.exe", "/bin/sh", "C:/x.exe", "a\\b.exe", ""] {
            assert!(
                check_launch("k", &spec(Some(program), None)).is_err(),
                "{program}"
            );
        }
        assert!(check_launch("k", &spec(None, None)).is_err());
        assert!(check_launch("k", &spec(Some("a.exe"), Some("hashlink"))).is_err());
        assert!(check_launch("k", &spec(None, Some("java>=8"))).is_err());
    }

    #[test]
    fn refuse_une_variante_en_double() {
        let mut twice = spec(Some("a.exe"), None);
        let variant = Variant {
            id: "directx".to_string(),
            args: Vec::new(),
        };
        twice.variants = vec![variant.clone(), variant];
        assert!(check_launch("k", &twice).is_err());
    }

    #[test]
    fn controle_le_dossier_des_sauvegardes() {
        let ok = BTreeMap::from([("windows".to_string(), "%APPDATA%/Galaxy55".to_string())]);
        assert!(check_saves(&ok).is_ok());
        for bad in ["/etc", "~/../x", "%APPDATA%/..", "C:/x"] {
            let saves = BTreeMap::from([("linux".to_string(), bad.to_string())]);
            assert!(check_saves(&saves).is_err(), "{bad}");
        }
    }
}
