use std::collections::BTreeMap;

/// Système et architecture de la machine, sous la forme utilisée par les clés du catalogue.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Platform {
    pub os: &'static str,
    pub arch: &'static str,
}

impl Platform {
    pub fn current() -> Self {
        Self {
            os: std::env::consts::OS,
            arch: std::env::consts::ARCH,
        }
    }

    /// `windows-x86_64` correspond exactement ; `linux-*` vaut pour toute architecture.
    pub fn matches(&self, key: &str) -> bool {
        match key.split_once('-') {
            Some((os, arch)) => os == self.os && (arch == "*" || arch == self.arch),
            None => false,
        }
    }

    /// Cherche l'entrée de la plateforme ; une clé exacte passe avant un joker.
    pub fn find<'a, T>(&self, entries: &'a BTreeMap<String, T>) -> Option<&'a T> {
        let exact = format!("{}-{}", self.os, self.arch);
        entries.get(&exact).or_else(|| {
            entries
                .iter()
                .find(|(key, _)| self.matches(key))
                .map(|(_, value)| value)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LINUX_X64: Platform = Platform {
        os: "linux",
        arch: "x86_64",
    };

    #[test]
    fn reconnait_les_cles_exactes_et_les_jokers() {
        assert!(LINUX_X64.matches("linux-x86_64"));
        assert!(LINUX_X64.matches("linux-*"));
        assert!(!LINUX_X64.matches("linux-aarch64"));
        assert!(!LINUX_X64.matches("macos-*"));
        assert!(!LINUX_X64.matches("linux"));
    }

    #[test]
    fn prefere_la_cle_exacte() {
        let entries = BTreeMap::from([
            ("linux-*".to_string(), "joker"),
            ("linux-x86_64".to_string(), "exacte"),
        ]);
        assert_eq!(LINUX_X64.find(&entries), Some(&"exacte"));
    }

    #[test]
    fn ne_trouve_rien_pour_un_autre_systeme() {
        let entries = BTreeMap::from([("windows-x86_64".to_string(), 1)]);
        assert_eq!(LINUX_X64.find(&entries), None);
    }
}
