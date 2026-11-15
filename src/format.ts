/** Date de publication lisible, dans la langue de l'interface. */
export function formatDate(iso: string, locale: string): string {
  return new Intl.DateTimeFormat(locale, { dateStyle: "long" }).format(new Date(iso));
}

/** Taille de fichier en Mo, avec une décimale. */
export function formatSize(bytes: number, locale: string): string {
  return new Intl.NumberFormat(locale, {
    style: "unit",
    unit: "megabyte",
    maximumFractionDigits: 1,
  }).format(bytes / 1_000_000);
}
