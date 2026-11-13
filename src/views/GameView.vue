<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";
import { computed, ref, toRef, watch } from "vue";
import { useI18n } from "vue-i18n";
import { type GameSummary, type GameVersion, getCatalog, getGameVersions } from "../api/launcher";
import VersionItem from "../components/VersionItem.vue";
import { useGameIcon } from "../composables/useGameIcon";
import { errorMessage } from "../errors";
import { pickLocalized } from "../localized";

const props = defineProps<{ id: string }>();
const i18n = useI18n();
const icon = useGameIcon(toRef(() => props.id));
const game = ref<GameSummary | null>(null);
const versions = ref<GameVersion[] | null>(null);
const error = ref<string | null>(null);
const loading = ref(false);

const name = computed(() => (game.value ? pickLocalized(game.value.name, i18n.locale.value) : ""));
const description = computed(() =>
  game.value ? pickLocalized(game.value.description, i18n.locale.value) : "",
);

async function load(refresh: boolean): Promise<void> {
  loading.value = true;
  error.value = null;
  try {
    const catalog = await getCatalog();
    game.value = catalog.games.find((g) => g.id === props.id) ?? null;
    versions.value = game.value ? await getGameVersions(props.id, refresh) : [];
  } catch (e) {
    error.value = errorMessage(e, i18n);
  } finally {
    loading.value = false;
  }
}

watch(() => props.id, () => load(false), { immediate: true });
</script>

<template>
  <section>
    <RouterLink to="/games" class="back">← {{ $t("game.back") }}</RouterLink>
    <header v-if="game" class="game-head">
      <img v-if="icon" :src="icon" alt="" class="icon" />
      <div>
        <h1>{{ name }}</h1>
        <p class="description">{{ description }}</p>
        <button type="button" class="button-ghost" @click="openUrl(game.repoUrl)">
          {{ $t("game.source") }}
        </button>
      </div>
    </header>
    <p v-else-if="!loading && !error" class="error">{{ $t("errors.unknownGame") }}</p>

    <header v-if="game" class="page-head">
      <h2>{{ $t("game.versions") }}</h2>
      <button type="button" class="button-ghost" :disabled="loading" @click="load(true)">
        {{ $t("common.refresh") }}
      </button>
    </header>
    <p v-if="error" class="error">{{ error }}</p>
    <p v-else-if="loading && versions === null" class="muted">{{ $t("game.loadingVersions") }}</p>
    <template v-else-if="game && versions">
      <p v-if="!game.available || versions.length === 0" class="muted">{{ $t("game.noVersions") }}</p>
      <p v-else class="muted">{{ $t("game.installSoon") }}</p>
      <ol class="versions">
        <VersionItem
          v-for="(v, index) in versions"
          :key="v.tag"
          :version="v"
          :latest="index === 0"
          :repo-url="game.repoUrl"
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

.game-head {
  display: flex;
  gap: 24px;
  margin-bottom: 32px;
}

.icon {
  flex: none;
  width: 128px;
  height: 128px;
  border-radius: var(--radius);
}

.description {
  max-width: 640px;
  color: var(--text-muted);
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
