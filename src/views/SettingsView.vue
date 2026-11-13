<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import { useGamesLayout } from "../composables/useGamesLayout";
import type { GamesLayout } from "../gamesLayout";
import {
  detectLocale,
  LANGUAGE_NAMES,
  type Locale,
  saveLocale,
  savedLocale,
  SUPPORTED_LOCALES,
} from "../locale";

const { locale } = useI18n();
const layout = useGamesLayout();
const layouts: { value: GamesLayout; label: string }[] = [
  { value: "list", label: "settings.layoutList" },
  { value: "grid", label: "settings.layoutGrid" },
];

// Chaîne vide = suivre la langue du système.
const choice = ref<Locale | "">(savedLocale(localStorage) ?? "");

function onLanguageChange(): void {
  const selected = choice.value === "" ? null : choice.value;
  saveLocale(localStorage, selected);
  locale.value = selected ?? detectLocale(navigator.languages);
}
</script>

<template>
  <section class="settings">
    <h1>{{ $t("settings.title") }}</h1>
    <label class="field">
      <span>{{ $t("settings.language") }}</span>
      <select v-model="choice" @change="onLanguageChange">
        <option value="">{{ $t("settings.systemLanguage") }}</option>
        <option v-for="code in SUPPORTED_LOCALES" :key="code" :value="code" :lang="code">
          {{ LANGUAGE_NAMES[code] }}
        </option>
      </select>
    </label>
    <fieldset class="field">
      <legend>{{ $t("settings.gamesLayout") }}</legend>
      <div class="choices">
        <label v-for="option in layouts" :key="option.value" class="choice">
          <input v-model="layout" type="radio" name="games-layout" :value="option.value" />
          {{ $t(option.label) }}
        </label>
      </div>
    </fieldset>
  </section>
</template>

<style scoped>
.settings {
  display: flex;
  flex-direction: column;
  gap: 24px;
}

.settings h1 {
  margin-bottom: 0;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  max-width: 320px;
  margin: 0;
  padding: 0;
  border: none;
}

legend {
  margin-bottom: 6px;
  padding: 0;
}

select {
  padding: 8px 10px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-elevated);
  color: var(--text);
  font: inherit;
}

.choices {
  display: flex;
  gap: 8px;
}

.choice {
  display: flex;
  flex: 1;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-elevated);
  cursor: pointer;
}

.choice:has(input:checked) {
  border-color: var(--accent);
}

.choice input {
  accent-color: var(--accent);
}
</style>
