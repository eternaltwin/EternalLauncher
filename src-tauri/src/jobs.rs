use crate::error::AppError;
use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Installations en cours (pour les annuler) et jeux lancés (pour ne pas les lancer deux fois).
#[derive(Default)]
pub struct Jobs {
    installs: Mutex<HashMap<String, Arc<AtomicBool>>>,
    running: Mutex<HashSet<String>>,
}

/// Tant qu'il existe, l'installation est « en cours » ; il la retire en disparaissant,
/// même si elle échoue en route.
pub struct InstallGuard<'a> {
    jobs: &'a Jobs,
    key: String,
    cancel: Arc<AtomicBool>,
}

impl InstallGuard<'_> {
    pub fn cancel_flag(&self) -> &AtomicBool {
        &self.cancel
    }
}

impl Drop for InstallGuard<'_> {
    fn drop(&mut self) {
        if let Ok(mut installs) = self.jobs.installs.lock() {
            installs.remove(&self.key);
        }
    }
}

pub fn install_key(game_id: &str, tag: &str) -> String {
    format!("{game_id}@{tag}")
}

impl Jobs {
    pub fn start_install(&self, key: String) -> Result<InstallGuard<'_>, AppError> {
        let mut installs = self.installs.lock().map_err(|_| AppError::Busy)?;
        if installs.contains_key(&key) {
            return Err(AppError::Busy);
        }
        let cancel = Arc::new(AtomicBool::new(false));
        installs.insert(key.clone(), cancel.clone());
        Ok(InstallGuard {
            jobs: self,
            key,
            cancel,
        })
    }

    pub fn cancel_install(&self, key: &str) {
        if let Ok(installs) = self.installs.lock()
            && let Some(cancel) = installs.get(key)
        {
            cancel.store(true, Ordering::Relaxed);
        }
    }

    pub fn start_game(&self, game_id: &str) -> Result<(), AppError> {
        let mut running = self.running.lock().map_err(|_| AppError::AlreadyRunning)?;
        if !running.insert(game_id.to_string()) {
            return Err(AppError::AlreadyRunning);
        }
        Ok(())
    }

    pub fn finish_game(&self, game_id: &str) {
        if let Ok(mut running) = self.running.lock() {
            running.remove(game_id);
        }
    }

    pub fn is_running(&self, game_id: &str) -> bool {
        self.running
            .lock()
            .is_ok_and(|running| running.contains(game_id))
    }

    pub fn running(&self) -> Vec<String> {
        self.running
            .lock()
            .map(|running| running.iter().cloned().collect())
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn une_version_ne_s_installe_qu_une_fois_a_la_fois() {
        let jobs = Jobs::default();
        let guard = jobs.start_install(install_key("jeu", "v1")).unwrap();
        assert!(matches!(
            jobs.start_install(install_key("jeu", "v1")),
            Err(AppError::Busy)
        ));
        jobs.cancel_install(&install_key("jeu", "v1"));
        assert!(guard.cancel_flag().load(Ordering::Relaxed));
        drop(guard);
        assert!(jobs.start_install(install_key("jeu", "v1")).is_ok());
    }

    #[test]
    fn un_jeu_ne_se_lance_qu_une_fois() {
        let jobs = Jobs::default();
        jobs.start_game("jeu").unwrap();
        assert!(matches!(
            jobs.start_game("jeu"),
            Err(AppError::AlreadyRunning)
        ));
        assert_eq!(jobs.running(), ["jeu"]);
        jobs.finish_game("jeu");
        assert!(!jobs.is_running("jeu"));
    }
}
