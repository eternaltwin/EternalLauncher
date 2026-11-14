export const SUPPORTED_LOCALES = ["fr", "en", "es", "de", "pt-BR"] as const;

export type Locale = (typeof SUPPORTED_LOCALES)[number];

export const FALLBACK_LOCALE: Locale = "en";

// Chaque langue est nommée dans sa propre langue.
export const LANGUAGE_NAMES: Record<Locale, string> = {
  fr: "Français",
  en: "English",
  es: "Español",
  de: "Deutsch",
  "pt-BR": "Português (Brasil)",
};

const STORAGE_KEY = "locale";

function isSupported(value: string): value is Locale {
  return (SUPPORTED_LOCALES as readonly string[]).includes(value);
}

// Ramène une langue du système (« fr-CA », « pt-PT », « de »…) à une langue gérée.
export function matchLocale(tag: string): Locale | null {
  const lower = tag.toLowerCase();
  if (lower.startsWith("pt")) {
    return "pt-BR";
  }
  const base = lower.split("-")[0];
  return isSupported(base) ? base : null;
}

export function detectLocale(preferred: readonly string[]): Locale {
  for (const tag of preferred) {
    const locale = matchLocale(tag);
    if (locale !== null) {
      return locale;
    }
  }
  return FALLBACK_LOCALE;
}

export function savedLocale(storage: Pick<Storage, "getItem">): Locale | null {
  try {
    const value = storage.getItem(STORAGE_KEY);
    return value !== null && isSupported(value) ? value : null;
  } catch {
    return null;
  }
}

export function saveLocale(storage: Pick<Storage, "setItem" | "removeItem">, locale: Locale | null): void {
  try {
    if (locale === null) {
      storage.removeItem(STORAGE_KEY);
    } else {
      storage.setItem(STORAGE_KEY, locale);
    }
  } catch {
    // Stockage indisponible : le choix ne vaut que pour cette session.
  }
}
