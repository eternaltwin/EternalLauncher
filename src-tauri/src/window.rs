use std::error::Error;
use tauri::webview::NewWindowResponse;
use tauri::{App, Url, WebviewWindowBuilder};

/// Crée la fenêtre principale décrite dans `tauri.conf.json` (où `create` vaut `false`), en
/// la gardant sur l'interface du launcher : aucune page distante ne s'y affiche jamais, et un
/// lien ouvert « dans une nouvelle fenêtre » (clic du milieu, menu contextuel) part dans le
/// navigateur du système s'il est en https.
pub fn create_main(app: &App) -> Result<(), Box<dyn Error>> {
    let config = app
        .config()
        .app
        .windows
        .iter()
        .find(|window| window.label == "main")
        .ok_or("la fenêtre « main » manque dans tauri.conf.json")?;
    let dev_url = if tauri::is_dev() {
        app.config().build.dev_url.clone()
    } else {
        None
    };
    WebviewWindowBuilder::from_config(app, config)?
        .on_navigation(move |url| is_app_url(url, dev_url.as_ref()))
        .on_new_window(|url, _| {
            if url.scheme() == "https" {
                let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
            }
            NewWindowResponse::Deny
        })
        .build()?;
    Ok(())
}

/// L'interface du launcher : `tauri://localhost` (macOS, Linux), `http://tauri.localhost`
/// (Windows), ou le serveur de `pnpm tauri dev`.
fn is_app_url(url: &Url, dev_url: Option<&Url>) -> bool {
    match (url.scheme(), url.host_str()) {
        ("tauri", Some("localhost")) | ("http" | "https", Some("tauri.localhost")) => true,
        ("about", _) => url.as_str() == "about:blank",
        _ => dev_url.is_some_and(|dev| dev.origin() == url.origin()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn url(text: &str) -> Url {
        Url::parse(text).unwrap()
    }

    #[test]
    fn reste_sur_l_interface_du_launcher() {
        for ok in [
            "tauri://localhost/index.html",
            "http://tauri.localhost/#/games/galaxy55",
            "https://tauri.localhost/",
            "about:blank",
        ] {
            assert!(is_app_url(&url(ok), None), "{ok}");
        }
    }

    #[test]
    fn refuse_toute_page_distante() {
        for bad in [
            "https://github.com/eternaltwin/EternalLauncher",
            "http://localhost:1420/",
            "https://tauri.localhost.evil.example/",
            "file:///C:/Windows/System32/",
            "data:text/html,<script>alert(1)</script>",
            "about:srcdoc",
        ] {
            assert!(!is_app_url(&url(bad), None), "{bad}");
        }
    }

    #[test]
    fn accepte_le_serveur_de_developpement_seulement_en_dev() {
        let dev = url("http://localhost:1420");
        assert!(is_app_url(
            &url("http://localhost:1420/#/settings"),
            Some(&dev)
        ));
        assert!(!is_app_url(&url("http://localhost:1421/"), Some(&dev)));
        assert!(!is_app_url(&url("http://localhost:1420/"), None));
    }
}
