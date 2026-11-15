import { useI18n } from "vue-i18n";

/** Nom affiché d'une variante de lancement (`directx` → « DirectX »), sinon son identifiant. */
export function useVariantLabel(): (id: string) => string {
  const { t, te } = useI18n();
  return (id) => (te(`variants.${id}`) ? t(`variants.${id}`) : id);
}

/** Quand choisir cette variante, en une phrase (infobulle du bouton). */
export function useVariantHelp(): (id: string) => string | undefined {
  const { t, te } = useI18n();
  return (id) => (te(`variantsHelp.${id}`) ? t(`variantsHelp.${id}`) : undefined);
}
