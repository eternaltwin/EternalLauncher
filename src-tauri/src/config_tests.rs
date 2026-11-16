//! La configuration de sécurité de `tauri.conf.json` et des permissions : la CI échoue si
//! une modification l'affaiblit par mégarde.

use serde_json::Value;

const TAURI_CONF: &str = include_str!("../tauri.conf.json");
const CAPABILITY: &str = include_str!("../capabilities/default.json");

fn json(text: &str) -> Value {
    serde_json::from_str(text).unwrap()
}

#[test]
fn la_mise_a_jour_exige_une_signature_liee_a_sa_version() {
    let updater = &json(TAURI_CONF)["plugins"]["updater"];
    assert_eq!(updater["requireSignedVersion"], true);
    assert_ne!(updater["allowDowngrades"], true);
}

#[test]
fn la_fenetre_ne_charge_que_l_interface_du_launcher() {
    let conf = json(TAURI_CONF);
    let csp = conf["app"]["security"]["csp"].as_str().unwrap();
    for directive in [
        "default-src 'self'",
        "object-src 'none'",
        "frame-src 'none'",
        "base-uri 'none'",
        "form-action 'none'",
    ] {
        assert!(csp.contains(directive), "CSP sans {directive}");
    }
    // La fenêtre est créée par `window::create_main`, avec sa garde de navigation.
    assert_eq!(conf["app"]["windows"][0]["create"], false);
}

#[test]
fn l_interface_n_a_que_les_permissions_qu_elle_utilise() {
    let capability = json(CAPABILITY);
    let permissions = capability["permissions"].as_array().unwrap();
    for permission in permissions {
        let id = permission
            .as_str()
            .or_else(|| permission["identifier"].as_str())
            .unwrap();
        assert!(!id.ends_with(":default"), "ensemble trop large : {id}");
    }
    let opener = permissions
        .iter()
        .find(|p| p["identifier"] == "opener:allow-open-url")
        .unwrap();
    assert_eq!(opener["allow"], serde_json::json!([{ "url": "https://*" }]));
}
