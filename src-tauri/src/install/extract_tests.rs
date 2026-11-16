use super::*;
use std::io::Write;
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

fn temp_dir(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("el-extract-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn options() -> SimpleFileOptions {
    SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored)
}

fn make_zip(dir: &Path, entries: &[(&str, &[u8])]) -> std::path::PathBuf {
    let path = dir.join("archive.zip");
    let mut writer = ZipWriter::new(std::fs::File::create(&path).unwrap());
    for (name, content) in entries {
        writer.start_file(*name, options()).unwrap();
        writer.write_all(content).unwrap();
    }
    writer.finish().unwrap();
    path
}

/// Extrait une archive faite de `entries` ; vrai si elle est refusée comme `BadArchive`.
fn refused(name: &str, entries: &[(&str, &[u8])]) -> bool {
    let dir = temp_dir(name);
    let zip = make_zip(&dir, entries);
    let result = extract(&zip, &dir.join("v0.8"));
    std::fs::remove_dir_all(&dir).unwrap();
    matches!(result, Err(AppError::BadArchive(_)))
}

#[test]
fn decompresse_une_archive_ordinaire() {
    let dir = temp_dir("ok");
    let zip = make_zip(
        &dir,
        &[("galaxy55.hl", b"code"), ("res/texture.png", b"png")],
    );
    let dest = dir.join("v0.8");
    extract(&zip, &dest).unwrap();
    assert_eq!(std::fs::read(dest.join("galaxy55.hl")).unwrap(), b"code");
    assert_eq!(
        std::fs::read(dest.join("res").join("texture.png")).unwrap(),
        b"png"
    );
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn refuse_un_chemin_qui_sort_du_dossier() {
    let dir = temp_dir("slip");
    let zip = make_zip(&dir, &[("ok.txt", b"ok"), ("../evil.txt", b"x")]);
    let dest = dir.join("v0.8");
    assert!(matches!(extract(&zip, &dest), Err(AppError::BadArchive(_))));
    assert!(!dir.join("evil.txt").exists());
    std::fs::remove_dir_all(&dir).unwrap();
    assert!(refused("slip-profond", &[("a/../../evil.txt", b"x")]));
}

#[test]
fn garde_un_chemin_absolu_dans_le_dossier() {
    let dir = temp_dir("absolu");
    let zip = make_zip(&dir, &[("/racine.txt", b"x")]);
    let dest = dir.join("v0.8");
    extract(&zip, &dest).unwrap();
    assert!(dest.join("racine.txt").is_file());
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn refuse_un_lien_symbolique() {
    let dir = temp_dir("lien");
    let path = dir.join("archive.zip");
    let mut writer = ZipWriter::new(std::fs::File::create(&path).unwrap());
    writer
        .add_symlink("lien", "/etc/passwd", options())
        .unwrap();
    writer.finish().unwrap();
    let result = extract(&path, &dir.join("v0.8"));
    assert!(matches!(result, Err(AppError::BadArchive(_))));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[test]
fn refuse_une_archive_trop_grosse_ou_trop_nombreuse() {
    let dir = temp_dir("bombe");
    let zip = make_zip(&dir, &[("a", b"12345"), ("b", b"67890")]);
    let dest = dir.join("v");
    let small = Limits {
        total_bytes: 8,
        entries: 10,
    };
    assert!(matches!(
        extract_with(&zip, &dest, &small),
        Err(AppError::BadArchive(_))
    ));
    let few = Limits {
        total_bytes: 1024,
        entries: 1,
    };
    assert!(matches!(
        extract_with(&zip, &dest, &few),
        Err(AppError::BadArchive(_))
    ));
    std::fs::remove_dir_all(&dir).unwrap();
}

#[cfg(windows)]
#[test]
fn refuse_les_noms_reserves_de_windows() {
    for name in ["CON", "res/aux.txt", "jeu.exe:flux", "nom.", "nom "] {
        assert!(refused("windows", &[(name, b"x")]), "{name}");
    }
}

#[test]
fn refuse_un_fichier_qui_n_est_pas_une_archive() {
    let dir = temp_dir("pas-zip");
    let path = dir.join("faux.zip");
    std::fs::write(&path, b"ceci n'est pas un zip").unwrap();
    assert!(matches!(
        extract(&path, &dir.join("v")),
        Err(AppError::BadArchive(_))
    ));
    std::fs::remove_dir_all(&dir).unwrap();
}
