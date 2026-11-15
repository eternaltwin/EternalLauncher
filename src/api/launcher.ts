import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export type Localized = Record<string, string>;

export type CatalogOrigin = "remote" | "cache" | "embedded";

export interface GameSummary {
  id: string;
  kind: "native";
  name: Localized;
  description: Localized;
  repo: string;
  repoUrl: string;
  available: boolean;
  variants: string[];
}

export interface CatalogView {
  origin: CatalogOrigin;
  platform: string;
  games: GameSummary[];
}

export interface GameVersion {
  tag: string;
  version: string;
  title: string | null;
  prerelease: boolean;
  publishedAt: string | null;
  notes: string;
  url: string;
  asset: {
    name: string;
    size: number;
    sha256: string | null;
    downloadUrl: string;
  };
}

export interface InstalledVersion {
  tag: string;
  version: string;
  asset: string;
  sha256: string;
  installedAt: number;
}

export interface InstalledState {
  versions: InstalledVersion[];
  lastPlayed: string | null;
  running: boolean;
  hasSaves: boolean;
}

export interface InstallProgress {
  gameId: string;
  tag: string;
  phase: "download" | "extract" | "done";
  received: number;
  total: number;
}

/** Erreur renvoyée par le cœur Rust (voir `src-tauri/src/error.rs`). */
export interface LauncherError {
  code: string;
  message: string;
  data: Record<string, string | number | null> | null;
}

export function isLauncherError(value: unknown): value is LauncherError {
  return typeof value === "object" && value !== null && "code" in value && "message" in value;
}

export const getCatalog = (refresh = false): Promise<CatalogView> =>
  invoke("get_catalog", { refresh });

export const getGameVersions = (id: string, refresh = false): Promise<GameVersion[]> =>
  invoke("get_game_versions", { id, refresh });

export const getGameIcon = (id: string): Promise<string> => invoke("get_game_icon", { id });

export const getInstalled = (id: string): Promise<InstalledState> =>
  invoke("get_installed", { id });

export const installVersion = (id: string, tag: string): Promise<InstalledVersion> =>
  invoke("install_version", { id, tag });

export const cancelInstall = (id: string, tag: string): Promise<void> =>
  invoke("cancel_install", { id, tag });

export const uninstallVersion = (id: string, tag: string): Promise<void> =>
  invoke("uninstall_version", { id, tag });

export const launchGame = (id: string, tag: string, variant: string | null): Promise<void> =>
  invoke("launch_game", { id, tag, variant });

export const openInstallDir = (id: string, tag: string): Promise<void> =>
  invoke("open_install_dir", { id, tag });

export const openSavesDir = (id: string): Promise<void> => invoke("open_saves_dir", { id });

export const onInstallProgress = (handler: (p: InstallProgress) => void): Promise<UnlistenFn> =>
  listen<InstallProgress>("install-progress", (event) => handler(event.payload));

/** Un jeu démarre ou se ferme : `gameId` en charge utile. */
export const onGameState = (handler: (gameId: string) => void): Promise<UnlistenFn> =>
  Promise.all(
    ["game-started", "game-exited"].map((name) =>
      listen<string>(name, (event) => handler(event.payload)),
    ),
  ).then((unlisten) => () => unlisten.forEach((stop) => stop()));
