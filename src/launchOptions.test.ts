import { describe, expect, it } from "vitest";
import { readShowAlternatives, writeShowAlternatives } from "./launchOptions";

describe("lancements alternatifs", () => {
  it("sont masqués par défaut", () => {
    expect(readShowAlternatives({ getItem: () => null })).toBe(false);
  });

  it("restent affichés une fois activés", () => {
    const data = new Map<string, string>();
    writeShowAlternatives({ setItem: (key, value) => void data.set(key, value) }, true);
    expect(readShowAlternatives({ getItem: (key) => data.get(key) ?? null })).toBe(true);
  });
});
