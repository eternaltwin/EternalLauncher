import MarkdownIt from "markdown-it";

// Notes de version écrites sur GitHub : le HTML brut est affiché comme du texte, les images
// ne sont pas chargées (aucun contenu distant dans la fenêtre), et markdown-it refuse déjà
// les liens `javascript:`, `vbscript:`, `file:` et `data:`.
const markdown = new MarkdownIt({ html: false, linkify: true, breaks: false });
markdown.disable(["image"]);

export function renderMarkdown(text: string): string {
  return markdown.render(text);
}

/**
 * Adresse à ouvrir dans le navigateur du système pour un clic dans les notes.
 * Un lien relatif (`CHANGELOG.md`) se lit depuis `base`, le dépôt au tag de la version.
 * Seul le https passe ; une ancre ou un lien illisible ne donne rien.
 */
export function externalLink(target: EventTarget | null, base?: string): string | null {
  if (!(target instanceof Element)) {
    return null;
  }
  const href = target.closest("a")?.getAttribute("href") ?? null;
  if (href === null || href.startsWith("#")) {
    return null;
  }
  try {
    const url = new URL(href, base);
    return url.protocol === "https:" ? url.href : null;
  } catch {
    return null;
  }
}
