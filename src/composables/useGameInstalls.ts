import { onUnmounted, reactive, type Ref, ref, watch } from "vue";
import { useI18n } from "vue-i18n";
import {
  cancelInstall,
  getInstalled,
  type InstalledState,
  type InstallProgress,
  installVersion,
  launchGame,
  onGameState,
  onInstallProgress,
  openInstallDir,
  openSavesDir,
  uninstallVersion,
} from "../api/launcher";
import { errorMessage, isCancelled } from "../errors";

/** Versions installées d'un jeu, installations en cours et actions (installer, jouer…). */
export function useGameInstalls(id: Ref<string>) {
  const i18n = useI18n();
  const installed = ref<InstalledState | null>(null);
  const progress = reactive(new Map<string, InstallProgress>());
  const error = ref<string | null>(null);

  async function refresh(): Promise<void> {
    installed.value = await getInstalled(id.value).catch(() => null);
  }

  // Une erreur s'affiche en haut de la fiche ; une annulation n'en est pas une.
  async function run(action: () => Promise<unknown>): Promise<void> {
    error.value = null;
    try {
      await action();
    } catch (e) {
      if (!isCancelled(e)) {
        error.value = errorMessage(e, i18n);
      }
    } finally {
      await refresh();
    }
  }

  const unlisten = [
    onInstallProgress((p) => {
      if (p.gameId === id.value && p.phase !== "done") {
        progress.set(p.tag, p);
      }
    }),
    onGameState((gameId) => {
      if (gameId === id.value) {
        void refresh();
      }
    }),
  ];
  onUnmounted(() => unlisten.forEach((pending) => void pending.then((stop) => stop())));
  watch(id, refresh, { immediate: true });

  return {
    installed,
    progress,
    error,
    install: (tag: string, size: number) => {
      progress.set(tag, { gameId: id.value, tag, phase: "download", received: 0, total: size });
      return run(() => installVersion(id.value, tag)).finally(() => progress.delete(tag));
    },
    cancel: (tag: string) => cancelInstall(id.value, tag),
    remove: (tag: string) => run(() => uninstallVersion(id.value, tag)),
    play: (tag: string, variant: string | null) => run(() => launchGame(id.value, tag, variant)),
    openDir: (tag: string) => run(() => openInstallDir(id.value, tag)),
    openSaves: () => run(() => openSavesDir(id.value)),
  };
}
