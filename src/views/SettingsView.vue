<script setup lang="ts">
import { ref } from "vue";
import { useI18n } from "vue-i18n";
import {
  detectLocale,
  LANGUAGE_NAMES,
  type Locale,
  saveLocale,
  savedLocale,
  SUPPORTED_LOCALES,
} from "../locale";

const { locale } = useI18n();

// Chaîne vide = suivre la langue du système.
const choice = ref<Locale | "">(savedLocale(localStorage) ?? "");

function onLanguageChange(): void {
  const selected = choice.value === "" ? null : choice.value;
  saveLocale(localStorage, selected);
  locale.value = selected ?? detectLocale(navigator.languages);
}
</script>

<template>
  <section>
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
  </section>
</template>

<style scoped>
.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  max-width: 320px;
}

select {
  padding: 8px 10px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-elevated);
  color: var(--text);
  font: inherit;
}
</style>
