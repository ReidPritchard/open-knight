<script setup lang="ts">
import "primeicons/primeicons.css";
import { onMounted, watch } from "vue";
import LayoutRenderer from "./components/AppLayout/LayoutRenderer.vue";
import { useGameStore } from "./stores/game";
import { useUIStore } from "./stores/ui";
import DynamicDialog from "primevue/dynamicdialog";

const gameStore = useGameStore();
const uiStore = useUIStore();

onMounted(async () => {
  await gameStore.updateGames();
  await gameStore.fetchSelectedGame();
});

watch(
  () => uiStore.layout,
  (newLayout) => {
    localStorage.setItem("app-layout", JSON.stringify(newLayout));
  },
  { deep: true },
);
</script>

<template>
  <LayoutRenderer :layout="uiStore.layout" />
  <DynamicDialog />
</template>

<style>
:root {
  font-family: "Noto Sans Mono", monospace;
  font-optical-sizing: auto;
  font-weight: 400;
  font-style: normal;
  font-variation-settings: "wdth" 100;

  color: var(--p-primary-color);
  background-color: var(--p-content-background);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;

  --modifiedBoardWidth: 500px;
}

html,
body,
#app {
  height: 100vh;
  width: 100vw;

  max-height: 100vh;
  max-width: 100vw;

  margin: 0;
  padding: 0;

  display: flex;
  flex-direction: column;
}
</style>
