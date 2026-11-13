use super::*;

const FIXTURE: &str = include_str!("../../fixtures/galaxy55-releases.json");

fn releases() -> Vec<Release> {
    serde_json::from_str(FIXTURE).unwrap()
}

fn spec(asset: &str) -> PlatformSpec {
    PlatformSpec {
        asset: asset.to_string(),
    }
}

const GALAXY55: VersionFilter = VersionFilter {
    min_version: Some("0.4"),
    prereleases: true,
};

#[test]
fn propose_les_versions_de_galaxy55_a_partir_de_la_0_4() {
    let versions = game_versions(&releases(), &GALAXY55, &spec("galaxy55-offline-{tag}.zip"));
    let tags: Vec<_> = versions.iter().map(|v| v.tag.as_str()).collect();
    assert_eq!(tags, ["v0.8.1", "v0.8", "v0.7", "v0.6", "v0.5", "v0.4"]);
}

#[test]
fn choisit_l_asset_de_la_plateforme_et_son_empreinte() {
    let versions = game_versions(
        &releases(),
        &GALAXY55,
        &spec("galaxy55-offline-{tag}-linux-mac.zip"),
    );
    let v08 = versions.iter().find(|v| v.tag == "v0.8").unwrap();
    assert_eq!(v08.version, "0.8");
    assert_eq!(v08.asset.name, "galaxy55-offline-v0.8-linux-mac.zip");
    assert_eq!(
        v08.asset.sha256.as_deref(),
        Some("67038f02279e99b6d31f6988ae77ffc5a0c5c657bf60810073a4a13e6c78cda0")
    );
    assert!(v08.prerelease);
}

#[test]
fn ignore_les_preversions_si_le_jeu_n_en_veut_pas() {
    let filter = VersionFilter {
        min_version: None,
        prereleases: false,
    };
    // Toutes les releases de Galaxy 55 sont des préversions.
    assert!(game_versions(&releases(), &filter, &spec("galaxy55-offline-{tag}.zip")).is_empty());
}

#[test]
fn ignore_les_brouillons_et_les_tags_illisibles() {
    let mut list = releases();
    list[0].draft = true;
    list[1].tag_name = "latest".to_string();
    let versions = game_versions(&list, &GALAXY55, &spec("galaxy55-offline-{tag}.zip"));
    assert_eq!(versions.first().map(|v| v.tag.as_str()), Some("v0.7"));
}

#[test]
fn ignore_une_release_sans_l_asset_attendu() {
    let versions = game_versions(&releases(), &GALAXY55, &spec("autre-{tag}.zip"));
    assert!(versions.is_empty());
}
