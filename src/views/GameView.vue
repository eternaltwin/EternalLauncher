<script setup lang="ts">
import { computed, ref, toRef, watch } from "vue";
import { useI18n } from "vue-i18n";
import { type GameSummary, type GameVersion, getCatalog, getGameVersions } from "../api/launcher";
import DowngradeWarning from "../components/DowngradeWarning.vue";
import GameHeader, { type MainAction } from "../components/GameHeader.vue";
import VersionItem from "../components/VersionItem.vue";
import { useGameIcon } from "../composables/useGameIcon";
import { useGameInstalls } from "../composables/useGameInstalls";
import { useShowAlternatives } from "../composables/useShowAlternatives";
import { errorMessage } from "../errors";
import { hasUpdate, isOlder } from "../versions";

const props = defineProps<{ id: string }>();
const i18n = useI18n();
const id = toRef(() => props.id);
const icon = useGameIcon(id);
const game = ref<GameSummary | null>(null);
const versions = ref<GameVersion[] | null>(null);
const loadError = ref<string | null>(null);
const loading = ref(false);
const installs = useGameInstalls(id);
const showAlternatives = useShowAlternatives();
const variants = computed(() => (showAlternatives.value ? (game.value?.variants ?? []) : []));
const { installed, progress } = installs;
const pending = ref<{ tag: string; variant: string | null } | null>(null);

const installedTags = computed(() => new Set(installed.value?.versions.map((v) => v.tag)));
const running = computed(() => installed.value?.running ?? false);
const lastPlayed = computed(() => installed.value?.lastPlayed ?? null);
const label = (tag: string | null) => versions.value?.find((v) => v.tag === tag)?.version ?? tag ?? "";

const mainAction = computed<MainAction>(() => {
  const list = versions.value ?? [];
  const playable = list.find((v) => installedTags.value.has(v.tag));
  if (running.value) return { kind: "busy", label: "running" };
  if (list[0] && progress.has(list[0].tag)) return { kind: "busy", label: "installing" };
  if (playable && hasUpdate(list, installedTags.value)) {
    return { kind: "update", version: list[0].version, installed: playable.version };
  }
  if (playable) return { kind: "play", version: playable.version };
  return list[0] ? { kind: "install", version: list[0].version } : { kind: "none" };
});

async function load(refresh: boolean): Promise<void> {
  loading.value = true;
  loadError.value = null;
  try {
    const catalog = await getCatalog();
    game.value = catalog.games.find((g) => g.id === props.id) ?? null;
    versions.value = game.value ? await getGameVersions(props.id, refresh) : [];
  } catch (e) {
    loadError.value = errorMessage(e, i18n);
  } finally {
    loading.value = false;
  }
}

// Lancer une version plus ancienne que la dernière jouée demande confirmation (sauvegardes).
function requestPlay(tag: string, variant: string | null, confirmed = false): void {
  if (!confirmed && isOlder(versions.value ?? [], tag, lastPlayed.value)) {
    pending.value = { tag, variant };
    return;
  }
  pending.value = null;
  void installs.play(tag, variant);
}

function playLatest(variant: string | null): void {
  const playable = versions.value?.find((v) => installedTags.value.has(v.tag));
  if (playable) requestPlay(playable.tag, variant);
}

function installLatest(): void {
  const latest = versions.value?.[0];
  if (latest) void installs.install(latest.tag, latest.asset.size);
}

watch(id, () => load(false), { immediate: true });
</script>

<template>
  <section>
    <RouterLink to="/games" class="back">← {{ $t("game.back") }}</RouterLink>
    <GameHeader
      v-if="game"
      :game="game"
      :icon="icon"
      :action="mainAction"
      :variants="variants"
      :has-saves="installed?.hasSaves ?? false"
      @play="playLatest"
      @install="installLatest"
      @update="installLatest"
      @open-saves="installs.openSaves"
    />
    <p v-else-if="!loading && !loadError" class="error">{{ $t("errors.unknownGame") }}</p>
    <p v-if="installs.error.value" class="error">{{ installs.error.value }}</p>
    <DowngradeWarning
      v-if="pending"
      :version="label(pending.tag)"
      :last-played="label(lastPlayed)"
      :has-saves="installed?.hasSaves ?? false"
      @confirm="requestPlay(pending.tag, pending.variant, true)"
      @dismiss="pending = null"
      @open-saves="installs.openSaves"
    />

    <header v-if="game" class="page-head">
      <h2>{{ $t("game.versions") }}</h2>
      <button type="button" class="button-ghost" :disabled="loading" @click="load(true)">
        {{ $t("common.refresh") }}
      </button>
    </header>
    <p v-if="loadError" class="error">{{ loadError }}</p>
    <p v-else-if="loading && versions === null" class="muted">{{ $t("game.loadingVersions") }}</p>
    <template v-else-if="game && versions">
      <p v-if="!game.available || versions.length === 0" class="muted">{{ $t("game.noVersions") }}</p>
      <ol class="versions">
        <VersionItem
          v-for="(v, index) in versions"
          :key="v.tag"
          :version="v"
          :latest="index === 0"
          :repo-url="game.repoUrl"
          :installed="installedTags.has(v.tag)"
          :progress="progress.get(v.tag)"
          :running="running"
          :variants="variants"
          @install="installs.install(v.tag, v.asset.size)"
          @cancel="installs.cancel(v.tag)"
          @play="(variant) => requestPlay(v.tag, variant)"
          @open-dir="installs.openDir(v.tag)"
          @remove="installs.remove(v.tag)"
        />
      </ol>
    </template>
  </section>
</template>

<style scoped>
.back {
  display: inline-block;
  margin-bottom: 16px;
  color: var(--text-muted);
  text-decoration: none;
}

.back:hover {
  color: var(--text);
}

h2 {
  margin: 0;
  font-size: 1.2rem;
}

.versions {
  display: flex;
  flex-direction: column;
  gap: 10px;
  margin: 0;
  padding: 0;
  list-style: none;
}
</style>
