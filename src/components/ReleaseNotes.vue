<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";
import { computed } from "vue";
import { externalLink, renderMarkdown } from "../markdown";

const props = defineProps<{ notes: string; base: string }>();
const html = computed(() => renderMarkdown(props.notes));

// Un lien ne doit jamais remplacer le launcher dans sa fenêtre : il part dans le navigateur.
function onClick(event: MouseEvent): void {
  if (event.target instanceof Element && event.target.closest("a")) {
    event.preventDefault();
    const url = externalLink(event.target, props.base);
    if (url !== null) {
      void openUrl(url);
    }
  }
}
</script>

<template>
  <!-- Markdown rendu sans HTML brut ni images : voir markdown.ts. -->
  <div class="notes" @click="onClick" v-html="html" />
</template>

<style scoped>
.notes {
  color: var(--text-muted);
  overflow-wrap: anywhere;
}

.notes :deep(h1),
.notes :deep(h2),
.notes :deep(h3),
.notes :deep(h4) {
  margin: 16px 0 8px;
  color: var(--text);
  font-size: 1rem;
}

.notes :deep(ul) {
  padding-left: 20px;
}

.notes :deep(code) {
  padding: 1px 4px;
  border-radius: 4px;
  background: var(--bg-hover);
}
</style>
