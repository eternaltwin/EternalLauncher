import type { GameVersion } from "./api/launcher";

/**
 * Vrai si `tag` est plus ancienne que la dernière version jouée. Les versions sont
 * triées de la plus récente à la plus ancienne ; une version inconnue ne compte pas.
 */
export function isOlder(versions: readonly GameVersion[], tag: string, lastPlayed: string | null): boolean {
  if (lastPlayed === null) {
    return false;
  }
  const index = versions.findIndex((v) => v.tag === tag);
  const last = versions.findIndex((v) => v.tag === lastPlayed);
  return index !== -1 && last !== -1 && index > last;
}

/**
 * Vrai si une version plus ancienne est installée mais pas la plus récente :
 * le launcher propose alors de mettre à jour, sans l'imposer.
 */
export function hasUpdate(versions: readonly GameVersion[], installed: ReadonlySet<string>): boolean {
  const latest = versions[0];
  return latest !== undefined && installed.size > 0 && !installed.has(latest.tag);
}
