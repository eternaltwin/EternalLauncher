// @vitest-environment node
/// <reference types="node" />
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import pkg from "../package.json";

// La version du launcher vit dans package.json et dans src-tauri/Cargo.toml :
// les deux doivent rester identiques (voir RELEASING.md).
describe("version du launcher", () => {
  it("est la même dans package.json et Cargo.toml", () => {
    const cargo = readFileSync(resolve(process.cwd(), "src-tauri/Cargo.toml"), "utf8");
    const version = /^version = "([^"]+)"/m.exec(cargo)?.[1];
    expect(version).toBe(pkg.version);
  });
});
