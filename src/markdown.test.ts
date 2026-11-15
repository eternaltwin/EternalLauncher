// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { externalLink, renderMarkdown } from "./markdown";

describe("renderMarkdown", () => {
  it("met en forme le Markdown des notes de version", () => {
    expect(renderMarkdown("## Nouveautés\n\n- **eau**")).toContain("<strong>eau</strong>");
  });

  it("affiche le HTML brut comme du texte", () => {
    const html = renderMarkdown('<script>alert(1)</script><img src="x" onerror="y">');
    expect(html).not.toContain("<script");
    expect(html).not.toContain("<img");
  });

  it("ne charge pas les images", () => {
    expect(renderMarkdown("![capture](https://example.org/a.png)")).not.toContain("<img");
  });

  it("refuse les liens javascript", () => {
    expect(renderMarkdown("[x](javascript:alert(1))")).not.toContain("href");
  });
});

describe("externalLink", () => {
  function link(href: string): Element {
    const a = document.createElement("a");
    a.setAttribute("href", href);
    const span = document.createElement("span");
    a.appendChild(span);
    return span;
  }

  it("garde un lien https, même cliqué sur un élément du lien", () => {
    expect(externalLink(link("https://github.com/"))).toBe("https://github.com/");
  });

  it("lit un lien relatif depuis le dépôt au tag de la version", () => {
    const base = "https://github.com/Pv1E/galaxy55-offline/blob/v0.8/";
    expect(externalLink(link("CHANGELOG.md"), base)).toBe(`${base}CHANGELOG.md`);
    expect(externalLink(link("CHANGELOG.md"))).toBeNull();
  });

  it("ignore les autres liens", () => {
    expect(externalLink(link("http://exemple.org/"))).toBeNull();
    expect(externalLink(link("#ancre"))).toBeNull();
    expect(externalLink(null)).toBeNull();
  });
});
