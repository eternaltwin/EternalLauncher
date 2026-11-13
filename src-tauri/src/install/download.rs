use crate::error::AppError;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};

/// Ce que GitHub annonce pour l'asset : on n'accepte que ce fichier-là, octet pour octet.
pub struct Expected<'a> {
    pub url: &'a str,
    pub size: u64,
    pub sha256: &'a str,
}

/// Télécharge l'asset dans `dest` en calculant son empreinte au passage. Le fichier est
/// supprimé si le téléchargement est annulé, trop long ou d'une empreinte différente.
pub async fn download(
    http: &reqwest::Client,
    expected: &Expected<'_>,
    dest: &Path,
    cancel: &AtomicBool,
    mut progress: impl FnMut(u64),
) -> Result<(), AppError> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let result = fetch(http, expected, dest, cancel, &mut progress).await;
    if result.is_err() {
        let _ = std::fs::remove_file(dest);
    }
    result
}

async fn fetch(
    http: &reqwest::Client,
    expected: &Expected<'_>,
    dest: &Path,
    cancel: &AtomicBool,
    progress: &mut impl FnMut(u64),
) -> Result<(), AppError> {
    let mut response = http.get(expected.url).send().await?.error_for_status()?;
    let mut file = std::fs::File::create(dest)?;
    let mut hasher = Sha256::new();
    let mut received: u64 = 0;
    while let Some(chunk) = response.chunk().await? {
        if cancel.load(Ordering::Relaxed) {
            return Err(AppError::Cancelled);
        }
        received += chunk.len() as u64;
        if received > expected.size {
            return Err(AppError::DigestMismatch);
        }
        hasher.update(&chunk);
        file.write_all(&chunk)?;
        progress(received);
    }
    file.sync_all()?;
    if received != expected.size || !same_digest(&hex(&hasher.finalize()), expected.sha256) {
        return Err(AppError::DigestMismatch);
    }
    Ok(())
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn same_digest(computed: &str, expected: &str) -> bool {
    computed.eq_ignore_ascii_case(expected.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ecrit_l_empreinte_en_hexadecimal() {
        let digest = Sha256::digest(b"abc");
        assert_eq!(
            hex(&digest),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn compare_les_empreintes_sans_tenir_compte_de_la_casse() {
        assert!(same_digest("abcd", "ABCD"));
        assert!(!same_digest("abcd", "abce"));
    }
}
