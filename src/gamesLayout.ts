export type GamesLayout = "list" | "grid";

export const DEFAULT_LAYOUT: GamesLayout = "list";

const STORAGE_KEY = "gamesLayout";

/** Affichage choisi dans les Réglages ; des lignes par défaut. */
export function readLayout(storage: Pick<Storage, "getItem">): GamesLayout {
  try {
    return storage.getItem(STORAGE_KEY) === "grid" ? "grid" : DEFAULT_LAYOUT;
  } catch {
    return DEFAULT_LAYOUT;
  }
}

export function writeLayout(storage: Pick<Storage, "setItem">, layout: GamesLayout): void {
  try {
    storage.setItem(STORAGE_KEY, layout);
  } catch {
    // Stockage indisponible : le choix ne vaut que pour cette session.
  }
}
