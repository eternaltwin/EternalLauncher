<script setup lang="ts">
import { computed, onUnmounted, ref } from "vue";
import type { InstallProgress } from "../api/launcher";
import { useVariantHelp, useVariantLabel } from "../composables/useVariantLabel";

const props = defineProps<{
  installed: boolean;
  progress: InstallProgress | undefined;
  running: boolean;
  variants: string[];
}>();
const emit = defineEmits<{
  install: [];
  cancel: [];
  play: [variant: string | null];
  openDir: [];
  remove: [];
}>();
const variantLabel = useVariantLabel();
const variantHelp = useVariantHelp();

const percent = computed(() =>
  props.progress ? Math.floor((props.progress.received * 100) / Math.max(props.progress.total, 1)) : 0,
);

// Supprimer demande un second clic dans les 4 secondes.
const confirmRemove = ref(false);
let timer: ReturnType<typeof setTimeout> | undefined;
function onRemove(): void {
  if (confirmRemove.value) {
    confirmRemove.value = false;
    emit("remove");
    return;
  }
  confirmRemove.value = true;
  clearTimeout(timer);
  timer = setTimeout(() => (confirmRemove.value = false), 4000);
}
onUnmounted(() => clearTimeout(timer));
</script>

<template>
  <div class="actions">
    <template v-if="progress">
      <div
        class="progress"
        role="progressbar"
        :aria-valuenow="percent"
        aria-valuemin="0"
        aria-valuemax="100"
      >
        <div class="bar" :style="{ width: `${percent}%` }" />
      </div>
      <span class="status">
        {{ progress.phase === "extract" ? $t("game.extracting") : $t("game.downloading", { percent }) }}
      </span>
      <button v-if="progress.phase === 'download'" type="button" class="button-ghost" @click="emit('cancel')">
        {{ $t("game.cancel") }}
      </button>
    </template>
    <template v-else-if="installed">
      <button type="button" class="button-primary" :disabled="running" @click="emit('play', null)">
        {{ $t("game.play") }}
      </button>
      <button
        v-for="variant in variants"
        :key="variant"
        type="button"
        class="button-ghost"
        :disabled="running"
        :title="variantHelp(variant)"
        @click="emit('play', variant)"
      >
        {{ $t("game.playWith", { variant: variantLabel(variant) }) }}
      </button>
      <button type="button" class="button-ghost" @click="emit('openDir')">{{ $t("game.folder") }}</button>
      <button type="button" class="button-ghost danger" :disabled="running" @click="onRemove">
        {{ confirmRemove ? $t("game.confirmRemove") : $t("game.remove") }}
      </button>
    </template>
    <button v-else type="button" class="button-primary" @click="emit('install')">
      {{ $t("game.install") }}
    </button>
  </div>
</template>

<style scoped>
.actions {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
  margin-top: 10px;
}

.progress {
  flex: 1;
  min-width: 160px;
  max-width: 360px;
  height: 8px;
  overflow: hidden;
  border-radius: 999px;
  background: var(--bg-hover);
}

.bar {
  height: 100%;
  background: var(--orange);
  transition: width 0.2s;
}

.status {
  color: var(--text-muted);
  font-size: 0.9rem;
  font-variant-numeric: tabular-nums;
}

.danger:hover:not(:disabled) {
  border-color: var(--accent-strong);
}
</style>
