use semver::{Prerelease, Version};

/// Lit un numéro de version tel qu'on le trouve dans un tag : `v0.8`, `0.3.1`, `v1.0-beta.2`.
/// Les parties manquantes valent 0 ; un tag qui n'est pas une version (`latest`) donne `None`.
pub fn parse_version(text: &str) -> Option<Version> {
    let text = text.trim();
    let text = text.strip_prefix(['v', 'V']).unwrap_or(text);
    let (core, pre) = match text.split_once('-') {
        Some((core, pre)) => (core, Some(pre)),
        None => (text, None),
    };
    let mut parts = core.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().map_or(Some(0), |part| part.parse().ok())?;
    let patch = parts.next().map_or(Some(0), |part| part.parse().ok())?;
    if parts.next().is_some() {
        return None;
    }
    let mut version = Version::new(major, minor, patch);
    if let Some(pre) = pre {
        version.pre = Prerelease::new(pre).ok()?;
    }
    Some(version)
}

/// Le tag sans son « v » initial : c'est ce qu'on montre au joueur.
pub fn display_version(tag: &str) -> &str {
    tag.strip_prefix(['v', 'V']).unwrap_or(tag)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lit_les_tags_courts_et_complets() {
        assert_eq!(parse_version("v0.8"), Some(Version::new(0, 8, 0)));
        assert_eq!(parse_version("v0.3.1"), Some(Version::new(0, 3, 1)));
        assert_eq!(parse_version("2"), Some(Version::new(2, 0, 0)));
        assert_eq!(parse_version(" V1.2.3 "), Some(Version::new(1, 2, 3)));
    }

    #[test]
    fn garde_le_suffixe_de_preversion() {
        let beta = parse_version("v1.0-beta.2").unwrap();
        assert_eq!(beta.pre.as_str(), "beta.2");
        assert!(beta < Version::new(1, 0, 0));
    }

    #[test]
    fn refuse_ce_qui_n_est_pas_une_version() {
        assert_eq!(parse_version("latest"), None);
        assert_eq!(parse_version("v1.2.3.4"), None);
        assert_eq!(parse_version(""), None);
    }

    #[test]
    fn affiche_le_tag_sans_le_v() {
        assert_eq!(display_version("v0.8.1"), "0.8.1");
        assert_eq!(display_version("1.0"), "1.0");
    }
}
