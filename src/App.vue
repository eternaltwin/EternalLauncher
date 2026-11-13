<script setup lang="ts">
import AppSidebar from "./components/AppSidebar.vue";
</script>

<template>
  <div class="layout">
    <AppSidebar />
    <main class="content">
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
