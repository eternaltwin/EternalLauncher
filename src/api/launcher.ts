import { invoke } from "@tauri-apps/api/core";

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

export type ErrorCode =
  | "network"
  | "rateLimited"
  | "github"
  | "unknownGame"
  | "invalidCatalog"
  | "tooLarge"
  | "io";

/** Erreur renvoyée par le cœur Rust (voir `src-tauri/src/error.rs`). */
export interface LauncherError {
  code: ErrorCode;
  message: string;
  reset: number | null;
}

export function isLauncherError(value: unknown): value is LauncherError {
  return typeof value === "object" && value !== null && "code" in value && "message" in value;
}

export function getCatalog(refresh = false): Promise<CatalogView> {
  return invoke("get_catalog", { refresh });
}

export function getGameVersions(id: string, refresh = false): Promise<GameVersion[]> {
  return invoke("get_game_versions", { id, refresh });
}

export function getGameIcon(id: string): Promise<string> {
  return invoke("get_game_icon", { id });
}
