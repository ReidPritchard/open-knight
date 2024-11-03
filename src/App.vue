<script setup lang="ts">
import "primeicons/primeicons.css";
import { onMounted, watch } from "vue";
import { useGlobalState } from "./shared/store";
import LayoutRenderer from "./components/AppLayout/LayoutRenderer.vue";

const { layout, updateGames, fetchSelectedGame } = useGlobalState();

onMounted(async () => {
  await updateGames();
  await fetchSelectedGame();
});

watch(
  layout,
  (newLayout) => {
    localStorage.setItem("app-layout", JSON.stringify(newLayout));
  },
  { deep: true },
);
</script>

<template>
  <LayoutRenderer :layout="layout" />
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
}

html,
#app,
body {
  color: var(--p-primary-color);
  background-color: var(--p-content-background);

  padding: 0;
  margin: 0;
  border: 0;

  width: 100%;
  height: 100%;
}
</style>
