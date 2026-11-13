use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Texte par langue (`fr`, `en`, `es`, `de`, `pt-BR`) ; l'interface choisit la bonne.
pub type Localized = BTreeMap<String, String>;

/// Le fichier `catalog/games.json` tel qu'il est lu : chaque jeu est validé à part,
/// pour qu'une entrée qu'un ancien launcher ne comprend pas ne casse pas les autres.
#[derive(Debug, Deserialize)]
pub struct RawCatalog {
    pub schema: u32,
    pub games: Vec<serde_json::Value>,
}

#[derive(Clone, Debug)]
pub struct Catalog {
    pub games: Vec<Game>,
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum GameKind {
    Native,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub id: String,
    pub kind: GameKind,
    pub name: Localized,
    pub description: Localized,
    pub icon: String,
    pub min_launcher: Option<String>,
    pub source: Source,
    pub platforms: BTreeMap<String, PlatformSpec>,
    /// Dossier des sauvegardes par système (`windows`, `linux`, `macos`).
    #[serde(default)]
    pub saves: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum Source {
    #[serde(rename = "github-releases", rename_all = "camelCase")]
    GitHubReleases {
        repo: String,
        min_version: Option<String>,
        #[serde(default)]
        prereleases: bool,
    },
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct PlatformSpec {
    /// Nom de l'asset de la release, avec `{tag}` ou `{version}`.
    pub asset: String,
    /// Programme à lancer, relatif au dossier de la version (sans runtime).
    pub program: Option<String>,
    /// Runtime à trouver sur la machine, par exemple `hashlink>=1.16`.
    pub runtime: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub variants: Vec<Variant>,
}

/// Autre façon de lancer le jeu (DirectX…) : d'autres arguments.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Variant {
    pub id: String,
    pub args: Vec<String>,
}

impl PlatformSpec {
    /// Nom de l'asset pour un tag donné : `{tag}` = `v0.8`, `{version}` = `0.8`.
    pub fn asset_name(&self, tag: &str) -> String {
        self.asset
            .replace("{tag}", tag)
            .replace("{version}", crate::version::display_version(tag))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remplace_tag_et_version_dans_le_nom_de_l_asset() {
        let spec = PlatformSpec {
            asset: "jeu-{tag}-{version}.zip".to_string(),
            ..PlatformSpec::default()
        };
        assert_eq!(spec.asset_name("v0.8"), "jeu-v0.8-0.8.zip");
    }
}
