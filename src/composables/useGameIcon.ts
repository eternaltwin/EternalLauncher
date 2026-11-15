import { type Ref, ref, watchEffect } from "vue";
import { getGameIcon } from "../api/launcher";

// Les icônes ne changent pas pendant une session : une seule demande par jeu.
const icons = new Map<string, Promise<string | null>>();

function loadIcon(id: string): Promise<string | null> {
  let icon = icons.get(id);
  if (icon === undefined) {
    icon = getGameIcon(id).catch(() => null);
    icons.set(id, icon);
  }
  return icon;
}

export function useGameIcon(id: Ref<string>): Ref<string | null> {
  const icon = ref<string | null>(null);
  watchEffect(async () => {
    const current = id.value;
    const url = await loadIcon(current);
    if (current === id.value) {
      icon.value = url;
    }
  });
  return icon;
}
