import { describe, expect, it } from "vitest";
import { readLayout, writeLayout } from "./gamesLayout";

describe("affichage des jeux", () => {
  it("affiche des lignes par défaut", () => {
    expect(readLayout({ getItem: () => null })).toBe("list");
    expect(readLayout({ getItem: () => "n'importe quoi" })).toBe("list");
  });

  it("relit la grille choisie", () => {
    const data = new Map<string, string>();
    writeLayout({ setItem: (key, value) => void data.set(key, value) }, "grid");
    expect(readLayout({ getItem: (key) => data.get(key) ?? null })).toBe("grid");
  });

  it("supporte un stockage indisponible", () => {
    const broken = () => {
      throw new Error("indisponible");
    };
    expect(readLayout({ getItem: broken })).toBe("list");
    expect(() => writeLayout({ setItem: broken }, "grid")).not.toThrow();
  });
});
