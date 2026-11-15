import { describe, expect, it } from "vitest";
import { percentOf, readCheckAtStartup, writeCheckAtStartup } from "./updatePreferences";

describe("mises à jour du launcher", () => {
  it("sont recherchées au démarrage par défaut", () => {
    expect(readCheckAtStartup({ getItem: () => null })).toBe(true);
  });

  it("ne le sont plus une fois l'option décochée", () => {
    const data = new Map<string, string>();
    writeCheckAtStartup({ setItem: (key, value) => void data.set(key, value) }, false);
    expect(readCheckAtStartup({ getItem: (key) => data.get(key) ?? null })).toBe(false);
  });

  it("calcule la progression du téléchargement", () => {
    expect(percentOf(0, 0)).toBe(0);
    expect(percentOf(50, 200)).toBe(25);
    expect(percentOf(300, 200)).toBe(100);
  });
});
