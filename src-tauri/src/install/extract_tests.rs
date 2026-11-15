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

fn make_zip(dir: &Path, entries: &[(&str, &[u8])]) -> std::path::PathBuf {
    let path = dir.join("archive.zip");
    let mut writer = ZipWriter::new(std::fs::File::create(&path).unwrap());
    let options = SimpleFileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for (name, content) in entries {
        writer.start_file(*name, options).unwrap();
        writer.write_all(content).unwrap();
    }
    writer.finish().unwrap();
    path
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
