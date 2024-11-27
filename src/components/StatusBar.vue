<template>
  <div class="status-bar">
    <p>Status Bar</p>
    <p>{{ theme }}</p>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useGlobalStore } from "../stores";

const { ui } = useGlobalStore();

const theme = computed(() => ui.$state.theme);
</script>

<style scoped>
.status-bar {
  background-color: var(--p-content-background);

  display: flex;
  flex-direction: row;
  justify-content: space-between;

  border: 1px solid var(--p-panel-header-border-color);
  border-radius: 0.5rem;
}

.status-bar > * {
  margin: 0.5rem;
}
</style>
