<script setup lang="ts">
import { useI18n } from "vue-i18n";
import type { GameVersion, InstallProgress } from "../api/launcher";
import { formatDate, formatSize } from "../format";
import ReleaseNotes from "./ReleaseNotes.vue";
import VersionActions from "./VersionActions.vue";

defineProps<{
  version: GameVersion;
  latest: boolean;
  repoUrl: string;
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
const { locale } = useI18n();
</script>

<template>
  <li class="version" :class="{ installed, installing: progress !== undefined }">
    <div class="line">
      <strong class="number">{{ version.version }}</strong>
      <span v-if="latest" class="badge badge-accent">{{ $t("game.latest") }}</span>
      <span v-if="version.prerelease" class="badge">{{ $t("game.prerelease") }}</span>
      <span v-if="installed" class="badge badge-installed">{{ $t("game.installed") }}</span>
      <span class="meta">
        <template v-if="version.publishedAt">
          {{ $t("game.published", { date: formatDate(version.publishedAt, locale) }) }} ·
        </template>
        {{ formatSize(version.asset.size, locale) }}
      </span>
    </div>
    <VersionActions
      :installed="installed"
      :progress="progress"
      :running="running"
      :variants="variants"
      @install="emit('install')"
      @cancel="emit('cancel')"
      @play="(variant) => emit('play', variant)"
      @open-dir="emit('openDir')"
      @remove="emit('remove')"
    />
    <details>
      <summary>{{ $t("game.notes") }}</summary>
      <ReleaseNotes
        v-if="version.notes.trim() !== ''"
        :notes="version.notes"
        :base="`${repoUrl}/blob/${version.tag}/`"
      />
      <p v-else class="muted">{{ $t("game.noNotes") }}</p>
    </details>
  </li>
</template>

<style scoped>
.version {
  padding: 12px 16px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-elevated);
}

.version.installed {
  border-left: 3px solid var(--green);
}

.version.installing {
  border-left: 3px solid var(--orange);
}

.line {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}

.number {
  font-size: 1.05rem;
}

.badge-installed {
  border-color: var(--green);
  color: var(--green);
}

.meta {
  margin-left: auto;
  color: var(--text-muted);
  font-size: 0.9rem;
}

summary {
  margin-top: 8px;
  color: var(--text-muted);
  cursor: pointer;
}
</style>
