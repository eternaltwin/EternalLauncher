import { describe, expect, it } from "vitest";
import { detectLocale, matchLocale, saveLocale, savedLocale } from "./locale";

function memoryStorage(initial: Record<string, string> = {}) {
  const data = new Map(Object.entries(initial));
  return {
    data,
    getItem: (key: string) => data.get(key) ?? null,
    setItem: (key: string, value: string) => void data.set(key, value),
    removeItem: (key: string) => void data.delete(key),
  };
}

describe("matchLocale", () => {
  it("ramène une variante régionale à la langue gérée", () => {
    expect(matchLocale("fr-CA")).toBe("fr");
    expect(matchLocale("de-AT")).toBe("de");
    expect(matchLocale("EN-us")).toBe("en");
  });

  it("ramène tout portugais au portugais du Brésil", () => {
    expect(matchLocale("pt")).toBe("pt-BR");
    expect(matchLocale("pt-PT")).toBe("pt-BR");
  });

  it("refuse une langue non gérée", () => {
    expect(matchLocale("ja-JP")).toBeNull();
  });
});

describe("detectLocale", () => {
  it("prend la première langue gérée du système", () => {
    expect(detectLocale(["ja", "es-MX", "fr"])).toBe("es");
  });

  it("revient à l'anglais sans langue gérée", () => {
    expect(detectLocale(["ja", "ko"])).toBe("en");
    expect(detectLocale([])).toBe("en");
  });
});

describe("choix enregistré", () => {
  it("relit une langue enregistrée et ignore une valeur inconnue", () => {
    expect(savedLocale(memoryStorage({ locale: "de" }))).toBe("de");
    expect(savedLocale(memoryStorage({ locale: "klingon" }))).toBeNull();
  });

  it("efface le choix pour revenir à la langue du système", () => {
    const storage = memoryStorage({ locale: "de" });
    saveLocale(storage, null);
    expect(storage.data.has("locale")).toBe(false);
    saveLocale(storage, "pt-BR");
    expect(storage.data.get("locale")).toBe("pt-BR");
  });

  it("supporte un stockage indisponible", () => {
    const broken = {
      getItem: () => {
        throw new Error("indisponible");
      },
    };
    expect(savedLocale(broken)).toBeNull();
  });
});
