<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import type { GameSummary } from "../api/launcher";
import { useVariantHelp, useVariantLabel } from "../composables/useVariantLabel";
import { pickLocalized } from "../localized";

/** Ce que propose le grand bouton : jouer à la dernière version installée, sinon installer la dernière. */
export type MainAction =
  | { kind: "play"; version: string }
  | { kind: "update"; version: string; installed: string }
  | { kind: "install"; version: string }
  | { kind: "busy"; label: "running" | "installing" }
  | { kind: "none" };

const props = defineProps<{
  game: GameSummary;
  icon: string | null;
  action: MainAction;
  variants: string[];
  hasSaves: boolean;
}>();
const emit = defineEmits<{
  play: [variant: string | null];
  install: [];
  update: [];
  openSaves: [];
}>();
const { locale } = useI18n();
const variantLabel = useVariantLabel();
const variantHelp = useVariantHelp();
const name = computed(() => pickLocalized(props.game.name, locale.value));
const description = computed(() => pickLocalized(props.game.description, locale.value));
</script>

<template>
  <header class="game-head">
    <img v-if="icon" :src="icon" alt="" class="icon" />
    <div class="info">
      <h1>{{ name }}</h1>
      <p class="description">{{ description }}</p>
      <p v-if="action.kind === 'update'" class="update">
        {{ $t("game.updateAvailable", { version: action.version }) }}
      </p>
      <div class="buttons">
        <template v-if="action.kind === 'play'">
          <button type="button" class="button-primary" @click="emit('play', null)">
            {{ $t("game.playVersion", { version: action.version }) }}
          </button>
          <button
            v-for="variant in variants"
            :key="variant"
            type="button"
            class="button-ghost"
            :title="variantHelp(variant)"
            @click="emit('play', variant)"
          >
            {{ $t("game.playWith", { variant: variantLabel(variant) }) }}
          </button>
        </template>
        <template v-else-if="action.kind === 'update'">
          <button type="button" class="button-primary" @click="emit('update')">
            {{ $t("game.update") }}
          </button>
          <button type="button" class="button-ghost" @click="emit('play', null)">
            {{ $t("game.playVersion", { version: action.installed }) }}
          </button>
        </template>
        <button
          v-else-if="action.kind === 'install'"
          type="button"
          class="button-primary"
          @click="emit('install')"
        >
          {{ $t("game.installVersion", { version: action.version }) }}
        </button>
        <button v-else-if="action.kind === 'busy'" type="button" class="button-primary" disabled>
          {{ action.label === "running" ? $t("game.running") : $t("game.extracting") }}
        </button>
        <button v-if="hasSaves" type="button" class="button-ghost" @click="emit('openSaves')">
          {{ $t("game.openSaves") }}
        </button>
        <button type="button" class="button-ghost" @click="openUrl(game.repoUrl)">
          {{ $t("game.source") }}
        </button>
      </div>
    </div>
  </header>
</template>

<style scoped>
.game-head {
  display: flex;
  gap: 24px;
  margin-bottom: 24px;
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

.update {
  color: var(--gold);
  font-weight: 600;
}

.buttons {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 8px;
}
</style>
