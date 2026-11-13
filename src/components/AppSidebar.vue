<script setup lang="ts">
import { getVersion } from "@tauri-apps/api/app";
import { onMounted, ref } from "vue";
import logo from "../assets/logo.svg";

const version = ref<string | null>(null);

const links = [
  { to: "/games", label: "nav.games" },
  { to: "/account", label: "nav.account" },
  { to: "/settings", label: "nav.settings" },
];

onMounted(async () => {
  try {
    version.value = await getVersion();
  } catch {
    // Hors de Tauri (simple `pnpm dev` dans un navigateur) : pas de version à afficher.
  }
});
</script>

<template>
  <aside class="sidebar">
    <div class="brand">
      <img :src="logo" alt="" class="logo" />
      <span>EternalLauncher</span>
    </div>
    <nav :aria-label="$t('app.navigation')">
      <RouterLink v-for="link in links" :key="link.to" :to="link.to" class="link">
        {{ $t(link.label) }}
      </RouterLink>
    </nav>
    <p v-if="version" class="version">{{ $t("app.version", { version }) }}</p>
  </aside>
</template>

<style scoped>
.sidebar {
  display: flex;
  flex-direction: column;
  gap: 24px;
  padding: 24px 16px;
  background: var(--bg-elevated);
  border-right: 1px solid var(--border);
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
  font-weight: 600;
  font-size: 1.1rem;
}

.logo {
  width: 36px;
}

nav {
  display: flex;
  flex-direction: column;
  gap: 4px;
}

.link {
  padding: 8px 12px;
  border-radius: var(--radius);
  color: var(--text);
  text-decoration: none;
}

.link:hover {
  background: var(--bg-hover);
}

.link.router-link-active {
  background: var(--accent);
}

.version {
  margin: auto 0 0;
  color: var(--text-muted);
  font-size: 0.85rem;
}
</style>
