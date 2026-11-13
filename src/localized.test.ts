import { describe, expect, it } from "vitest";
import { pickLocalized } from "./localized";

describe("pickLocalized", () => {
  const text = { fr: "Bonjour", en: "Hello", de: "Hallo" };

  it("prend la langue de l'interface", () => {
    expect(pickLocalized(text, "de")).toBe("Hallo");
  });

  it("revient à l'anglais, puis au français", () => {
    expect(pickLocalized(text, "es")).toBe("Hello");
    expect(pickLocalized({ fr: "Bonjour" }, "pt-BR")).toBe("Bonjour");
  });

  it("prend la première langue fournie en dernier recours", () => {
    expect(pickLocalized({ es: "Hola" }, "de")).toBe("Hola");
    expect(pickLocalized({}, "fr")).toBe("");
  });
});
