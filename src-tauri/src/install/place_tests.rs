use super::*;
use crate::install::paths::MARKER;
use crate::install::registry::tests::installed;
use std::io::Write;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

fn temp_root(name: &str) -> std::path::PathBuf {
    let root = std::env::temp_dir().join(format!("el-place-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    root
}

fn make_zip(root: &Path, entries: &[(&str, &[u8])]) -> std::path::PathBuf {
    let path = root.join("jeu.zip");
    let mut writer = ZipWriter::new(std::fs::File::create(&path).unwrap());
    for (name, content) in entries {
        writer
            .start_file(*name, SimpleFileOptions::default())
            .unwrap();
        writer.write_all(content).unwrap();
    }
    writer.finish().unwrap();
    path
}

/// Ce que contient le dossier du jeu, dossiers cachés compris.
fn entries(paths: &GamePaths) -> Vec<String> {
    let mut names: Vec<_> = std::fs::read_dir(paths.dir())
        .unwrap()
        .map(|entry| entry.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    names
}

#[test]
fn installe_la_version_et_ne_laisse_rien_d_autre() {
    let root = temp_root("ok");
    let paths = GamePaths::new(&root, "jeu");
    let zip = make_zip(&root, &[("jeu.exe", b"exe")]);
    place(&paths, &zip, &installed("v1.0")).unwrap();
    let dir = paths.version("v1.0").unwrap();
    assert_eq!(std::fs::read(dir.join("jeu.exe")).unwrap(), b"exe");
    assert!(dir.join(MARKER).is_file());
    assert_eq!(entries(&paths), ["v1.0"]);
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn remplace_le_reste_d_une_suppression_interrompue() {
    let root = temp_root("reste");
    let paths = GamePaths::new(&root, "jeu");
    let leftover = paths.version("v1.0").unwrap();
    std::fs::create_dir_all(&leftover).unwrap();
    std::fs::write(leftover.join("vieux.txt"), b"x").unwrap();
    let zip = make_zip(&root, &[("jeu.exe", b"exe")]);
    place(&paths, &zip, &installed("v1.0")).unwrap();
    assert!(!leftover.join("vieux.txt").exists());
    assert!(leftover.join("jeu.exe").is_file());
    assert_eq!(entries(&paths), ["v1.0"]);
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn ne_touche_pas_a_une_version_deja_installee() {
    let root = temp_root("deja");
    let paths = GamePaths::new(&root, "jeu");
    let first = make_zip(&root, &[("jeu.exe", b"premier")]);
    place(&paths, &first, &installed("v1.0")).unwrap();
    let second = make_zip(&root, &[("jeu.exe", b"second")]);
    assert!(matches!(
        place(&paths, &second, &installed("v1.0")),
        Err(AppError::Busy)
    ));
    let exe = paths.version("v1.0").unwrap().join("jeu.exe");
    assert_eq!(std::fs::read(exe).unwrap(), b"premier");
    assert_eq!(entries(&paths), ["v1.0"]);
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn ne_garde_rien_d_une_archive_refusee() {
    let root = temp_root("refus");
    let paths = GamePaths::new(&root, "jeu");
    std::fs::create_dir_all(paths.dir()).unwrap();
    let zip = make_zip(&root, &[("ok.txt", b"ok"), ("../evil.txt", b"x")]);
    assert!(place(&paths, &zip, &installed("v1.0")).is_err());
    assert!(entries(&paths).is_empty());
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn supprime_une_version_installee_et_seulement_elle() {
    let root = temp_root("suppr");
    let paths = GamePaths::new(&root, "jeu");
    let zip = make_zip(&root, &[("jeu.exe", b"exe")]);
    place(&paths, &zip, &installed("v1.0")).unwrap();
    place(&paths, &zip, &installed("v2.0")).unwrap();
    remove(&paths, "v1.0").unwrap();
    assert_eq!(entries(&paths), ["v2.0"]);
    assert!(matches!(
        remove(&paths, "v1.0"),
        Err(AppError::NotInstalled)
    ));
    std::fs::remove_dir_all(&root).unwrap();
}

/// Sous Windows, un fichier ouvert dans le dossier (le jeu lancé par une session précédente
/// du launcher) bloque le renommage : la version reste entière au lieu d'être à moitié effacée.
#[cfg(windows)]
#[test]
fn laisse_la_version_entiere_si_un_fichier_est_ouvert() {
    let root = temp_root("ouvert");
    let paths = GamePaths::new(&root, "jeu");
    let zip = make_zip(&root, &[("jeu.exe", b"exe")]);
    place(&paths, &zip, &installed("v1.0")).unwrap();
    let open = std::fs::File::open(paths.version("v1.0").unwrap().join("jeu.exe")).unwrap();
    assert!(remove(&paths, "v1.0").is_err());
    assert!(registry::is_installed(&paths, "v1.0").unwrap());
    drop(open);
    remove(&paths, "v1.0").unwrap();
    std::fs::remove_dir_all(&root).unwrap();
}
