import { createI18n } from "vue-i18n";
import de from "./locales/de.json";
import en from "./locales/en.json";
import es from "./locales/es.json";
import fr from "./locales/fr.json";
import ptBR from "./locales/pt-BR.json";
import { detectLocale, FALLBACK_LOCALE, savedLocale } from "./locale";

export const messages = { fr, en, es, de, "pt-BR": ptBR };

export const i18n = createI18n({
  legacy: false,
  locale: savedLocale(localStorage) ?? detectLocale(navigator.languages),
  fallbackLocale: FALLBACK_LOCALE,
  messages,
});
