import { describe, expect, it } from "vitest";
import { messages } from "./i18n";
import { SUPPORTED_LOCALES } from "./locale";

function keys(value: unknown, prefix = ""): string[] {
  if (typeof value !== "object" || value === null) {
    return [prefix];
  }
  return Object.entries(value).flatMap(([key, child]) => keys(child, prefix === "" ? key : `${prefix}.${key}`));
}

describe("traductions", () => {
  const reference = keys(messages.fr).sort();

  it.each(SUPPORTED_LOCALES)("%s a exactement les clés du français", (locale) => {
    expect(keys(messages[locale]).sort()).toEqual(reference);
  });
});
