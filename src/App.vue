<script setup lang="ts">
import { onMounted } from "vue";
import AppSidebar from "./components/AppSidebar.vue";
import LauncherUpdateBanner from "./components/LauncherUpdateBanner.vue";
import { useLauncherUpdate } from "./composables/useLauncherUpdate";

const { checkAtStartup, checkForUpdate } = useLauncherUpdate();

// Une vérification au démarrage, si le joueur ne l'a pas désactivée ; elle n'installe rien.
onMounted(() => {
  if (checkAtStartup.value) {
    void checkForUpdate();
  }
});
</script>

<template>
  <div class="layout">
    <AppSidebar />
    <main class="content">
      <LauncherUpdateBanner />
      <RouterView />
    </main>
  </div>
</template>

<style scoped>
.layout {
  display: grid;
  grid-template-columns: 220px 1fr;
  height: 100vh;
}

.content {
  overflow-y: auto;
  padding: 32px 40px;
}
</style>
