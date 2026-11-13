import { relaunch } from "@tauri-apps/plugin-process";
import { check, type Update } from "@tauri-apps/plugin-updater";
import { ref, shallowRef, watch } from "vue";
import { percentOf, readCheckAtStartup, writeCheckAtStartup } from "../updatePreferences";

export type UpdateStatus = "idle" | "checking" | "upToDate" | "available" | "downloading" | "error";

// État partagé : le bandeau, les Réglages et la vérification au démarrage.
const status = ref<UpdateStatus>("idle");
const update = shallowRef<Update | null>(null);
const percent = ref(0);
const dismissed = ref(false);
const checkAtStartup = ref(readCheckAtStartup(localStorage));
watch(checkAtStartup, (value) => writeCheckAtStartup(localStorage, value));

/** Demande à GitHub si une version plus récente du launcher est publiée. N'installe rien. */
async function checkForUpdate(): Promise<void> {
  status.value = "checking";
  try {
    update.value = await check();
    status.value = update.value ? "available" : "upToDate";
    dismissed.value = false;
  } catch {
    status.value = "error";
  }
}

/** Seul le clic du joueur sur « Installer et redémarrer » arrive ici. */
async function installUpdate(): Promise<void> {
  const current = update.value;
  if (!current) {
    return;
  }
  status.value = "downloading";
  let total = 0;
  let received = 0;
  try {
    await current.downloadAndInstall((event) => {
      if (event.event === "Started") {
        total = event.data.contentLength ?? 0;
      } else if (event.event === "Progress") {
        received += event.data.chunkLength;
        percent.value = percentOf(received, total);
      }
    });
    await relaunch();
  } catch {
    status.value = "error";
  }
}

export function useLauncherUpdate() {
  return {
    status,
    update,
    percent,
    dismissed,
    checkAtStartup,
    checkForUpdate,
    installUpdate,
    dismiss: () => (dismissed.value = true),
  };
}
