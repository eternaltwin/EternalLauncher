use crate::catalog::model::PlatformSpec;
use crate::version::{display_version, parse_version};
use serde::{Deserialize, Serialize};

/// Une release telle que l'API GitHub la renvoie (seulement les champs utiles).
#[derive(Clone, Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub name: Option<String>,
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub prerelease: bool,
    pub published_at: Option<String>,
    pub body: Option<String>,
    pub html_url: String,
    #[serde(default)]
    pub assets: Vec<ReleaseAsset>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReleaseAsset {
    pub name: String,
    pub size: u64,
    /// `sha256:<hex>`, fourni par GitHub pour les assets envoyés depuis 2025.
    pub digest: Option<String>,
    pub browser_download_url: String,
}

/// Une version d'un jeu disponible pour cette machine.
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GameVersion {
    pub tag: String,
    pub version: String,
    pub title: Option<String>,
    pub prerelease: bool,
    pub published_at: Option<String>,
    pub notes: String,
    pub url: String,
    pub asset: VersionAsset,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct VersionAsset {
    pub name: String,
    pub size: u64,
    pub sha256: Option<String>,
    pub download_url: String,
}

pub struct VersionFilter<'a> {
    pub min_version: Option<&'a str>,
    pub prereleases: bool,
}

/// Les versions proposées : pas de brouillon, pas de préversion si le jeu n'en veut pas,
/// rien sous `minVersion`, et seulement celles qui ont l'asset de cette plateforme.
/// Les plus récentes d'abord.
pub fn game_versions(
    releases: &[Release],
    filter: &VersionFilter,
    spec: &PlatformSpec,
) -> Vec<GameVersion> {
    let min = filter.min_version.and_then(parse_version);
    let mut versions: Vec<_> = releases
        .iter()
        .filter(|release| !release.draft && (filter.prereleases || !release.prerelease))
        .filter_map(|release| {
            let version = parse_version(&release.tag_name)?;
            if min.as_ref().is_some_and(|min| &version < min) {
                return None;
            }
            let wanted = spec.asset_name(&release.tag_name);
            let asset = release.assets.iter().find(|asset| asset.name == wanted)?;
            Some((version, to_game_version(release, asset)))
        })
        .collect();
    versions.sort_by(|a, b| b.0.cmp(&a.0));
    versions.into_iter().map(|(_, version)| version).collect()
}

fn to_game_version(release: &Release, asset: &ReleaseAsset) -> GameVersion {
    GameVersion {
        tag: release.tag_name.clone(),
        version: display_version(&release.tag_name).to_string(),
        title: release.name.clone().filter(|name| !name.trim().is_empty()),
        prerelease: release.prerelease,
        published_at: release.published_at.clone(),
        notes: release.body.clone().unwrap_or_default(),
        url: release.html_url.clone(),
        asset: VersionAsset {
            name: asset.name.clone(),
            size: asset.size,
            sha256: asset
                .digest
                .as_deref()
                .and_then(|digest| digest.strip_prefix("sha256:"))
                .map(str::to_string),
            download_url: asset.browser_download_url.clone(),
        },
    }
}

#[cfg(test)]
#[path = "releases_tests.rs"]
mod tests;
