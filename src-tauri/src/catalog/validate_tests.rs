use super::*;
use crate::platform::Platform;
use serde_json::json;

const REPO_CATALOG: &str = include_str!("../../../catalog/games.json");

fn game(overrides: serde_json::Value) -> serde_json::Value {
    let mut base = json!({
        "id": "jeu",
        "kind": "native",
        "name": { "fr": "Jeu" },
        "description": { "fr": "Un jeu." },
        "icon": "icons/jeu.png",
        "source": { "type": "github-releases", "repo": "proprio/jeu" },
        "platforms": { "windows-x86_64": { "asset": "jeu-{tag}.zip", "program": "jeu.exe", "args": [] } }
    });
    for (key, value) in overrides.as_object().unwrap() {
        base[key] = value.clone();
    }
    base
}

fn catalog(games: Vec<serde_json::Value>) -> String {
    json!({ "schema": 1, "games": games }).to_string()
}

#[test]
fn le_catalogue_du_depot_est_valide() {
    let catalog = parse_strict(REPO_CATALOG).expect("catalog/games.json doit être valide");
    let galaxy = catalog.games.iter().find(|g| g.id == "galaxy55").unwrap();
    for (os, arch) in [
        ("windows", "x86_64"),
        ("linux", "x86_64"),
        ("macos", "aarch64"),
    ] {
        let platform = Platform { os, arch };
        assert!(platform.find(&galaxy.platforms).is_some(), "{os}-{arch}");
    }
}

#[test]
fn ignore_un_jeu_invalide_sans_perdre_les_autres() {
    let text = catalog(vec![
        game(json!({ "id": "bon" })),
        game(json!({ "id": "Mauvais Id" })),
        game(json!({ "id": "autre", "kind": "inconnu" })),
    ]);
    let ids: Vec<_> = parse(&text, "0.1.0")
        .unwrap()
        .games
        .into_iter()
        .map(|g| g.id)
        .collect();
    assert_eq!(ids, ["bon"]);
}

#[test]
fn ignore_un_jeu_qui_demande_un_launcher_plus_recent() {
    let text = catalog(vec![
        game(json!({ "id": "ancien", "minLauncher": "0.1" })),
        game(json!({ "id": "futur", "minLauncher": "2.0" })),
    ]);
    let ids: Vec<_> = parse(&text, "0.1.0")
        .unwrap()
        .games
        .into_iter()
        .map(|g| g.id)
        .collect();
    assert_eq!(ids, ["ancien"]);
}

#[test]
fn refuse_un_format_inconnu() {
    let text = json!({ "schema": 2, "games": [] }).to_string();
    assert!(matches!(
        parse(&text, "0.1.0"),
        Err(AppError::InvalidCatalog(_))
    ));
}

#[test]
fn refuse_les_chemins_et_depots_dangereux() {
    for bad in [
        json!({ "icon": "../secret.png" }),
        json!({ "icon": "icons/../../x.png" }),
        json!({ "source": { "type": "github-releases", "repo": "a/../b" } }),
        json!({ "source": { "type": "github-releases", "repo": "a/b?x=1" } }),
        json!({ "platforms": { "windows-x86_64": { "asset": "fixe.zip", "program": "jeu.exe" } } }),
        json!({ "platforms": { "windows-x86_64": { "asset": "{tag}.zip", "program": "../jeu.exe" } } }),
        json!({ "saves": { "windows": "C:/Windows" } }),
        json!({ "platforms": { "beos-x86_64": { "asset": "{tag}.zip", "program": "jeu.exe" } } }),
        json!({ "name": { "klingon": "Jeu" } }),
    ] {
        let text = catalog(vec![game(bad.clone())]);
        assert!(parse_strict(&text).is_err(), "{bad}");
    }
}

#[test]
fn refuse_un_id_en_double() {
    let text = catalog(vec![game(json!({})), game(json!({}))]);
    assert!(parse_strict(&text).is_err());
    assert_eq!(parse(&text, "0.1.0").unwrap().games.len(), 1);
}
