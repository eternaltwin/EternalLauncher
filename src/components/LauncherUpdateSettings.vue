<script setup lang="ts">
import { useLauncherUpdate } from "../composables/useLauncherUpdate";

const { status, update, checkAtStartup, checkForUpdate } = useLauncherUpdate();
</script>

<template>
  <fieldset class="field">
    <legend>{{ $t("settings.updates") }}</legend>
    <label class="choice">
      <input v-model="checkAtStartup" type="checkbox" />
      {{ $t("settings.checkAtStartup") }}
    </label>
    <div class="row">
      <button
        type="button"
        class="button-ghost"
        :disabled="status === 'checking' || status === 'downloading'"
        @click="checkForUpdate"
      >
        {{ $t("settings.checkNow") }}
      </button>
      <span v-if="status === 'checking'" class="muted">{{ $t("settings.checking") }}</span>
      <span v-else-if="status === 'upToDate'" class="muted">{{ $t("settings.upToDate") }}</span>
      <span v-else-if="status === 'available' && update" class="found">
        {{ $t("settings.updateFound", { version: update.version }) }}
      </span>
      <span v-else-if="status === 'error'" class="muted">{{ $t("settings.updateError") }}</span>
    </div>
  </fieldset>
</template>

<style scoped>
.field {
  display: flex;
  flex-direction: column;
  gap: 8px;
  max-width: 520px;
  margin: 0;
  padding: 0;
  border: none;
}

legend {
  margin-bottom: 6px;
  padding: 0;
}

.choice {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  border: 1px solid var(--border);
  border-radius: var(--radius);
  background: var(--bg-elevated);
  cursor: pointer;
}

.choice input {
  accent-color: var(--accent);
}

.row {
  display: flex;
  flex-wrap: wrap;
  align-items: center;
  gap: 10px;
}

.found {
  color: var(--green);
}
</style>
