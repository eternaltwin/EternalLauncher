<script setup lang="ts">
defineProps<{ version: string; lastPlayed: string; hasSaves: boolean }>();
const emit = defineEmits<{ confirm: []; dismiss: []; openSaves: [] }>();
</script>

<template>
  <div class="notice warning" role="alert">
    <p>
      <strong>{{ $t("game.downgradeTitle", { version, lastPlayed }) }}</strong>
      {{ $t("game.downgradeText") }}
    </p>
    <div class="buttons">
      <button type="button" class="button-primary" @click="emit('confirm')">
        {{ $t("game.playAnyway") }}
      </button>
      <button v-if="hasSaves" type="button" class="button-ghost" @click="emit('openSaves')">
        {{ $t("game.openSaves") }}
      </button>
      <button type="button" class="button-ghost" @click="emit('dismiss')">{{ $t("game.cancel") }}</button>
    </div>
  </div>
</template>

<style scoped>
.warning {
  margin-bottom: 16px;
  color: var(--text);
}

.warning p {
  margin: 0 0 10px;
}

.buttons {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
}
</style>
