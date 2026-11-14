const STORAGE_KEY = "showAlternativeLaunches";

/**
 * Lancements alternatifs (« Jouer (DirectX) »…) : pour un PC où le lancement normal échoue.
 * Masqués par défaut, ils s'activent dans les Réglages.
 */
export function readShowAlternatives(storage: Pick<Storage, "getItem">): boolean {
  try {
    return storage.getItem(STORAGE_KEY) === "true";
  } catch {
    return false;
  }
}

export function writeShowAlternatives(storage: Pick<Storage, "setItem">, show: boolean): void {
  try {
    storage.setItem(STORAGE_KEY, String(show));
  } catch {
    // Stockage indisponible : le choix ne vaut que pour cette session.
  }
}
