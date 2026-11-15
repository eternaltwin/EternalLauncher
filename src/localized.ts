import type { Localized } from "./api/launcher";

/**
 * Texte du catalogue dans la langue de l'interface, sinon en anglais, sinon en français,
 * sinon la première langue fournie.
 */
export function pickLocalized(text: Localized, locale: string): string {
  for (const candidate of [locale, "en", "fr"]) {
    const value = text[candidate];
    if (value !== undefined && value !== "") {
      return value;
    }
  }
  return Object.values(text)[0] ?? "";
}
