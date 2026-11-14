import { describe, expect, it } from "vitest";
import type { GameVersion } from "./api/launcher";
import { hasUpdate, isOlder } from "./versions";

const versions = ["v0.8.1", "v0.8", "v0.7"].map((tag) => ({ tag }) as GameVersion);

describe("isOlder", () => {
  it("signale une version plus ancienne que la dernière jouée", () => {
    expect(isOlder(versions, "v0.7", "v0.8")).toBe(true);
  });

  it("laisse passer la même version ou une plus récente", () => {
    expect(isOlder(versions, "v0.8", "v0.8")).toBe(false);
    expect(isOlder(versions, "v0.8.1", "v0.8")).toBe(false);
  });

  it("ne dit rien sans dernière version connue", () => {
    expect(isOlder(versions, "v0.7", null)).toBe(false);
    expect(isOlder(versions, "v0.7", "v9.9")).toBe(false);
  });
});

describe("hasUpdate", () => {
  it("propose la mise à jour quand seule une ancienne version est installée", () => {
    expect(hasUpdate(versions, new Set(["v0.8"]))).toBe(true);
  });

  it("ne propose rien si la plus récente est installée ou si rien ne l'est", () => {
    expect(hasUpdate(versions, new Set(["v0.8.1", "v0.8"]))).toBe(false);
    expect(hasUpdate(versions, new Set())).toBe(false);
    expect(hasUpdate([], new Set(["v0.8"]))).toBe(false);
  });
});
