<template>
  <section>
    <h1>{{ $t("games.title") }}</h1>
    <p class="muted">{{ $t("games.empty") }}</p>
  </section>
</template>

<style scoped>
.muted {
  color: var(--text-muted);
}
</style>
