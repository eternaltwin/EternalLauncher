import { type Ref, ref, watchEffect } from "vue";
import { getGameVersions, getInstalled } from "../api/launcher";
import { hasUpdate } from "../versions";

/** Pour la liste des jeux : une version plus récente que celles installées existe-t-elle ? */
export function useUpdateAvailable(id: Ref<string>): Ref<boolean> {
  const available = ref(false);
  watchEffect(async () => {
    const current = id.value;
    try {
      const [installed, versions] = await Promise.all([getInstalled(current), getGameVersions(current)]);
      if (current === id.value) {
        available.value = hasUpdate(versions, new Set(installed.versions.map((v) => v.tag)));
      }
    } catch {
      available.value = false;
    }
  });
  return available;
}
