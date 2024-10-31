<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import "primeicons/primeicons.css";
import { onMounted, ref, watch } from "vue";
import {
  apiExplorerStateToExplorerState,
  apiSelectedGameToGame,
} from "./shared/api-conversions";
import {
  IGame,
  ILayout,
  IWindowContainer,
  validateWindowContainer,
} from "./shared/types";
import { applicationLayout } from "./applicationLayout";
import LayoutRenderer from "./components/AppLayout/LayoutRenderer.vue";

const games = ref<IGame[]>([]);
const selectedGame = ref<IGame | null>(null);
const pgn = ref("");

async function updateGames() {
  const state: string = await invoke("get_explorer_state");
  const parsedState = apiExplorerStateToExplorerState(state);
  console.log("Parsed state:", parsedState);
  games.value = parsedState.games;
}

async function getSelectedGame() {
  // Can return "null"
  const response: string = await invoke("get_selected_game");
  // Parse "null" to null
  const game: string | null = response === "null" ? null : response;
  // Now parse the game (handles null as well)
  const parsedGame = apiSelectedGameToGame(game);
  selectedGame.value = parsedGame;
}

async function parsePgn() {
  await invoke("parse_pgn", { pgn: pgn.value });
  await updateGames();
  await getSelectedGame();

  // Clear the pgn input
  pgn.value = "";
}

onMounted(async () => {
  await updateGames();
  await getSelectedGame();

  // const savedLayout = localStorage.getItem('app-layout');
  // if (savedLayout) {
  //   layout.value = JSON.parse(savedLayout);
  // }
});

let menuVisible = ref(false);
let menuItems = ref([
  {
    label: "File",
    items: [
      {
        label: "Open",
        icon: "pi pi-folder-open",
        command: () => {
          console.log("Open");
        },
      },
      {
        label: "Save",
        icon: "pi pi-save",
        command: () => {
          console.log("Save");
        },
      },
    ],
  },
]);

async function emptyDb() {
  await invoke("empty_db");
  await updateGames();
}

const toggleTheme = () => {
  document.documentElement.classList.toggle("dark");
};

const layout = ref<ILayout>(applicationLayout);

// Helper to find a window in the layout by its id
// searches recursively through all children
function findWindowInLayout(windowId: string): ILayout | null {
  const isContainer = validateWindowContainer(layout.value);

  if (!isContainer.success) {
    return null;
  }

  const container = isContainer.data;
  const children = container.children;

  for (const child of children) {
    if (child?.id === windowId) {
      return child;
    }
  }

  return null;
}

function collapseWindow(windowId: string, collapsed: boolean) {
  console.log("Collapsing window:", windowId, collapsed);

  // Find the window in the layout
  const window = findWindowInLayout(windowId);
  if (window && validateWindowContainer(window).success) {
    (window as IWindowContainer).collapsed = !window.collapsed;
  }
}

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
