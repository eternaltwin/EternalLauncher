import type { Composer } from "vue-i18n";
import { isLauncherError } from "./api/launcher";

const RUNTIME_NAMES: Record<string, string> = { hashlink: "HashLink" };

const SIMPLE_CODES = [
  "network",
  "github",
  "unknownGame",
  "unknownVersion",
  "unavailable",
  "missingDigest",
  "digestMismatch",
  "badArchive",
  "busy",
  "notInstalled",
  "alreadyRunning",
  "launch",
  "noSaves",
  "io",
];

/** Un téléchargement annulé par le joueur n'est pas une erreur à afficher. */
export function isCancelled(error: unknown): boolean {
  return isLauncherError(error) && error.code === "cancelled";
}

/** Message à montrer au joueur pour une erreur du cœur Rust, dans sa langue. */
export function errorMessage(error: unknown, i18n: Pick<Composer, "t" | "locale">): string {
  if (!isLauncherError(error)) {
    return i18n.t("errors.generic");
  }
  const data = error.data ?? {};
  const runtime = RUNTIME_NAMES[String(data.runtime)] ?? String(data.runtime);
  switch (error.code) {
    case "rateLimited": {
      if (typeof data.reset !== "number") {
        return i18n.t("errors.rateLimitedSoon");
      }
      const time = new Intl.DateTimeFormat(i18n.locale.value, { timeStyle: "short" }).format(
        new Date(data.reset * 1000),
      );
      return i18n.t("errors.rateLimited", { time });
    }
    case "runtimeMissing":
      return i18n.t("errors.runtimeMissing", { runtime, min: data.min ?? "" });
    case "runtimeTooOld":
      return i18n.t("errors.runtimeTooOld", { runtime, found: data.found, min: data.min });
    default:
      return SIMPLE_CODES.includes(error.code)
        ? i18n.t(`errors.${error.code}`)
        : i18n.t("errors.generic");
  }
}
