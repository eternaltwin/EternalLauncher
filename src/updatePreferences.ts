const STORAGE_KEY = "checkLauncherUpdates";

/** Rechercher les mises à jour du launcher au démarrage : oui par défaut. */
export function readCheckAtStartup(storage: Pick<Storage, "getItem">): boolean {
  try {
    return storage.getItem(STORAGE_KEY) !== "false";
  } catch {
    return true;
  }
}

export function writeCheckAtStartup(storage: Pick<Storage, "setItem">, check: boolean): void {
  try {
    storage.setItem(STORAGE_KEY, String(check));
  } catch {
    // Stockage indisponible : le choix ne vaut que pour cette session.
  }
}

/** Pourcentage téléchargé ; 0 tant que la taille totale est inconnue. */
export function percentOf(received: number, total: number): number {
  return total > 0 ? Math.min(100, Math.floor((received * 100) / total)) : 0;
}
