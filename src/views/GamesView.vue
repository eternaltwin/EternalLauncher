<script setup lang="ts">
import { onMounted, ref } from "vue";
import { useI18n } from "vue-i18n";
import { type CatalogView, getCatalog } from "../api/launcher";
import GameCard from "../components/GameCard.vue";
import { useGamesLayout } from "../composables/useGamesLayout";
import { errorMessage } from "../errors";

const i18n = useI18n();
const layout = useGamesLayout();
const catalog = ref<CatalogView | null>(null);
const error = ref<string | null>(null);
const loading = ref(false);

async function load(refresh: boolean): Promise<void> {
  loading.value = true;
  error.value = null;
  try {
    catalog.value = await getCatalog(refresh);
  } catch (e) {
    error.value = errorMessage(e, i18n);
  } finally {
    loading.value = false;
  }
}

onMounted(() => load(false));
</script>

<template>
  <section>
    <header class="page-head">
      <h1>{{ $t("games.title") }}</h1>
      <button type="button" class="button-ghost" :disabled="loading" @click="load(true)">
        {{ $t("common.refresh") }}
      </button>
    </header>
    <p v-if="catalog && catalog.origin !== 'remote'" class="notice">{{ $t("games.offline") }}</p>
    <p v-if="error" class="error">{{ error }}</p>
    <p v-else-if="loading && !catalog" class="muted">{{ $t("games.loading") }}</p>
    <p v-else-if="catalog && catalog.games.length === 0" class="muted">{{ $t("games.empty") }}</p>
    <div v-if="catalog" :class="layout === 'list' ? 'rows' : 'grid'">
      <GameCard v-for="game in catalog.games" :key="game.id" :game="game" :layout="layout" />
    </div>
  </section>
</template>

<style scoped>
.rows {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(360px, 1fr));
  gap: 16px;
}
</style>
