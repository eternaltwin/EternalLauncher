<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";
import { computed } from "vue";
import { useLauncherUpdate } from "../composables/useLauncherUpdate";

const { status, update, percent, dismissed, installUpdate, dismiss } = useLauncherUpdate();
const visible = computed(
  () => update.value !== null && !dismissed.value && ["available", "downloading"].includes(status.value),
);
const notesUrl = computed(
  () => `https://github.com/eternaltwin/EternalLauncher/releases/tag/v${update.value?.version ?? ""}`,
);
</script>

<template>
  <div v-if="visible && update" class="banner" role="status">
    <template v-if="status === 'downloading'">
      <span>{{ $t("launcherUpdate.downloading", { percent }) }}</span>
    </template>
    <template v-else>
      <span>{{ $t("launcherUpdate.available", { version: update.version }) }}</span>
      <div class="buttons">
        <button type="button" class="button-primary" @click="installUpdate">
          {{ $t("launcherUpdate.install") }}
        </button>
        <button type="button" class="button-ghost" @click="openUrl(notesUrl)">
          {{ $t("launcherUpdate.notes") }}
        </button>
        <button type="button" class="button-ghost" @click="dismiss">
          {{ $t("launcherUpdate.later") }}
        </button>
      </div>
    </template>
  </div>
</template>

<style scoped>
.banner {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  margin-bottom: 24px;
  padding: 10px 14px;
  border: 1px solid var(--green);
  border-radius: var(--radius);
  background: rgb(63 179 107 / 10%);
}

.buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
</style>
