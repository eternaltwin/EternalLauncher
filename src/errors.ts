import type { Composer } from "vue-i18n";
import { isLauncherError } from "./api/launcher";

/** Message à montrer au joueur pour une erreur du cœur Rust, dans sa langue. */
export function errorMessage(error: unknown, i18n: Pick<Composer, "t" | "locale">): string {
  if (!isLauncherError(error)) {
    return i18n.t("errors.generic");
  }
  if (error.code === "rateLimited") {
    if (error.reset === null) {
      return i18n.t("errors.rateLimitedSoon");
    }
    const time = new Intl.DateTimeFormat(i18n.locale.value, { timeStyle: "short" }).format(
      new Date(error.reset * 1000),
    );
    return i18n.t("errors.rateLimited", { time });
  }
  const known = ["network", "github", "unknownGame"] as const;
  return (known as readonly string[]).includes(error.code)
    ? i18n.t(`errors.${error.code}`)
    : i18n.t("errors.generic");
}
