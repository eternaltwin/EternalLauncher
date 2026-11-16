use crate::error::AppError;
use sha2::{Digest, Sha256};
use std::fs::File;
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
    if !is_sha256(expected.sha256) {
        return Err(AppError::MissingDigest);
    }
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
    // Une taille annoncée différente : inutile de télécharger pour le constater.
    if response
        .content_length()
        .is_some_and(|length| length != expected.size)
    {
        return Err(AppError::DigestMismatch);
    }
    let mut sink = Sink::create(dest, expected)?;
    while let Some(chunk) = response.chunk().await? {
        progress(sink.write(&chunk, cancel)?);
    }
    sink.finish()
}

/// Le fichier en cours d'écriture : jamais plus d'octets que la taille annoncée, l'annulation
/// prise en compte à chaque morceau, et l'empreinte comparée à la fin.
pub struct Sink<'a> {
    file: File,
    hasher: Sha256,
    received: u64,
    expected: &'a Expected<'a>,
}

impl<'a> Sink<'a> {
    pub fn create(dest: &Path, expected: &'a Expected<'a>) -> Result<Self, AppError> {
        Ok(Self {
            file: File::create(dest)?,
            hasher: Sha256::new(),
            received: 0,
            expected,
        })
    }

    /// Ajoute un morceau ; renvoie le nombre d'octets reçus jusque-là.
    pub fn write(&mut self, chunk: &[u8], cancel: &AtomicBool) -> Result<u64, AppError> {
        if cancel.load(Ordering::Relaxed) {
            return Err(AppError::Cancelled);
        }
        self.received += chunk.len() as u64;
        if self.received > self.expected.size {
            return Err(AppError::DigestMismatch);
        }
        self.hasher.update(chunk);
        self.file.write_all(chunk)?;
        Ok(self.received)
    }

    pub fn finish(self) -> Result<(), AppError> {
        self.file.sync_all()?;
        let digest = hex(&self.hasher.finalize());
        if self.received != self.expected.size || !same_digest(&digest, self.expected.sha256) {
            return Err(AppError::DigestMismatch);
        }
        Ok(())
    }
}

pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Une empreinte sha256 : 64 chiffres hexadécimaux.
fn is_sha256(text: &str) -> bool {
    text.len() == 64 && text.chars().all(|c| c.is_ascii_hexdigit())
}

fn same_digest(computed: &str, expected: &str) -> bool {
    computed.eq_ignore_ascii_case(expected.trim())
}

#[cfg(test)]
#[path = "download_tests.rs"]
mod tests;
