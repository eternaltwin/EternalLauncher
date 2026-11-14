<script setup lang="ts">
import { computed, toRef } from "vue";
import { useI18n } from "vue-i18n";
import type { GameSummary } from "../api/launcher";
import { useGameIcon } from "../composables/useGameIcon";
import { useUpdateAvailable } from "../composables/useUpdateAvailable";
import type { GamesLayout } from "../gamesLayout";
import { pickLocalized } from "../localized";

const props = defineProps<{ game: GameSummary; layout: GamesLayout }>();
const { locale } = useI18n();
const id = toRef(() => props.game.id);
const icon = useGameIcon(id);
const update = useUpdateAvailable(id);
const name = computed(() => pickLocalized(props.game.name, locale.value));
const description = computed(() => pickLocalized(props.game.description, locale.value));
</script>

<template>
  <RouterLink :to="`/games/${game.id}`" class="card" :class="layout">
    <img v-if="icon" :src="icon" alt="" class="icon" />
    <div v-else class="icon placeholder" />
    <div class="body">
      <div class="title">
        <h2>{{ name }}</h2>
        <span v-if="!game.available" class="badge">{{ $t("games.unavailable") }}</span>
        <span v-if="update" class="badge badge-update">{{ $t("games.updateAvailable") }}</span>
      </div>
      <p class="description">{{ description }}</p>
    </div>
    <span v-if="layout === 'list'" class="chevron" aria-hidden="true">›</span>
  </RouterLink>
</template>

<style scoped>
.card {
  display: flex;
  gap: 16px;
  padding: 16px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-elevated);
  color: var(--text);
  text-decoration: none;
}

.card:hover {
  background: var(--bg-hover);
  border-color: var(--accent);
}

/* En lignes : toutes de la même hauteur, quelle que soit la description. */
.card.list {
  align-items: center;
  height: 104px;
}

.icon {
  flex: none;
  border-radius: var(--radius);
}

.list .icon {
  width: 72px;
  height: 72px;
}

.grid .icon {
  width: 96px;
  height: 96px;
}

.placeholder {
  background: var(--bg-hover);
}

.body {
  flex: 1;
  min-width: 0;
}

.title {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px 10px;
  margin-bottom: 4px;
}

h2 {
  margin: 0;
  font-size: 1.15rem;
}

.description {
  display: -webkit-box;
  margin: 0;
  overflow: hidden;
  color: var(--text-muted);
  -webkit-box-orient: vertical;
}

.list .description {
  -webkit-line-clamp: 2;
}

.grid .description {
  -webkit-line-clamp: 3;
}

.badge-update {
  border-color: var(--gold);
  color: var(--gold);
}

.chevron {
  flex: none;
  color: var(--text-muted);
  font-size: 1.8rem;
  line-height: 1;
}

.card:hover .chevron {
  color: var(--text);
}
</style>
