use super::*;

const URL: &str = "https://github.com/proprio/jeu/releases/download/v1/jeu.zip";

fn temp_file(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("el-download-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    dir.join("jeu.zip.partial")
}

fn sha256_of(bytes: &[u8]) -> String {
    hex(&Sha256::digest(bytes))
}

/// Écrit `chunks` comme s'ils arrivaient du réseau, puis vérifie le fichier.
fn receive(
    name: &str,
    expected: &Expected,
    chunks: &[&[u8]],
    cancel: bool,
) -> Result<(), AppError> {
    let dest = temp_file(name);
    let flag = AtomicBool::new(cancel);
    let result = (|| {
        let mut sink = Sink::create(&dest, expected)?;
        for chunk in chunks {
            sink.write(chunk, &flag)?;
        }
        sink.finish()
    })();
    std::fs::remove_dir_all(dest.parent().unwrap()).unwrap();
    result
}

#[test]
fn ecrit_l_empreinte_en_hexadecimal() {
    assert_eq!(
        sha256_of(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn compare_les_empreintes_sans_tenir_compte_de_la_casse() {
    assert!(same_digest("abcd", "ABCD"));
    assert!(!same_digest("abcd", "abce"));
}

#[test]
fn accepte_le_fichier_annonce_octet_pour_octet() {
    let sha = sha256_of(b"contenu");
    let expected = Expected {
        url: URL,
        size: 7,
        sha256: &sha,
    };
    assert!(receive("ok", &expected, &[b"cont", b"enu"], false).is_ok());
}

#[test]
fn s_arrete_des_que_la_taille_annoncee_est_depassee() {
    let sha = sha256_of(b"contenu");
    let expected = Expected {
        url: URL,
        size: 7,
        sha256: &sha,
    };
    let result = receive("trop", &expected, &[b"contenu", b"en trop"], false);
    assert!(matches!(result, Err(AppError::DigestMismatch)));
}

#[test]
fn refuse_un_fichier_tronque_ou_modifie() {
    let sha = sha256_of(b"contenu");
    let expected = Expected {
        url: URL,
        size: 7,
        sha256: &sha,
    };
    let short = receive("court", &expected, &[b"conten"], false);
    assert!(matches!(short, Err(AppError::DigestMismatch)));
    let other = receive("autre", &expected, &[b"CONTENU"], false);
    assert!(matches!(other, Err(AppError::DigestMismatch)));
}

#[test]
fn s_arrete_quand_le_joueur_annule() {
    let sha = sha256_of(b"contenu");
    let expected = Expected {
        url: URL,
        size: 7,
        sha256: &sha,
    };
    let result = receive("annule", &expected, &[b"contenu"], true);
    assert!(matches!(result, Err(AppError::Cancelled)));
}

#[test]
fn n_accepte_qu_une_empreinte_sha256_bien_formee() {
    assert!(is_sha256(&sha256_of(b"x")));
    for bad in ["", "abcd", &"z".repeat(64), &"a".repeat(65)] {
        assert!(!is_sha256(bad), "{bad}");
    }
}
