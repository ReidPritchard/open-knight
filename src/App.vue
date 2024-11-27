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
  <div class="app-root">
    <LayoutRenderer :layout="uiStore.layout" />
    <DynamicDialog />
  </div>
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
body {
  height: 100vh;
  width: 100vw;
  margin: 0;
  padding: 0;
  overflow: hidden;
}

#app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}

.app-root {
  height: 100vh;
  width: 100vw;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Root layout styles */
.app-root > :first-child {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Menu bar */
[data-container-id="menu-bar"] {
  flex-shrink: 0;
}

/* Status bar */
[data-container-id="status-bar"] {
  flex-shrink: 0;
}

/* Main area */
[data-container-id="main-area"] {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Content area */
[data-container-id="content-area"] {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow: hidden;
}

/* Center pane */
[data-container-id="center-pane"] {
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

/* Bottom panel container */
[data-container-id="bottom-bar"] {
  flex-shrink: 0;
}
</style>
