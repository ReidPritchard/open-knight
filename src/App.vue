<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import "primeicons/primeicons.css";
import Badge from "primevue/badge";
import Menubar from "primevue/menubar";
import { onMounted, ref } from "vue";
import AppLayout from "./components/AppLayout/AppLayout.vue";
import GameBoard from "./components/GameBoard.vue";
import GameExplorer from "./components/GameExplorer.vue";
import {
  apiExplorerStateToExplorerState,
  apiSelectedGameToGame,
} from "./shared/api-conversions";
import {
  IGame,
  ILayout,
  IWindowContainer,
  validateWindowContainer,
  WindowDirection,
  WindowDisplay,
} from "./shared/types";

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

const defaultLayout: ILayout = {
  id: "root",
  direction: WindowDirection.Horizontal,
  display: WindowDisplay.Split,
  size: 100,
  children: [
    // Left sidebar
    {
      id: "side-bar",
      direction: WindowDirection.Vertical,
      display: WindowDisplay.Accordion,
      size: 256,
      children: [
        // Game explorer
        {
          id: "game-explorer",
          size: 256,
          resizable: true,
          collapsed: false,
        },
      ],
    },
    // Center pane
    {
      id: "center-pane",
      direction: WindowDirection.Vertical,
      display: WindowDisplay.Tabs,
      size: 100,
      children: [
        // Menu bar
        {
          id: "menu-bar",
          size: 50,
          resizable: false,
          collapsed: false,
        },
        // Game board
        {
          id: "game-board",
          size: 500,
          resizable: true,
          collapsed: false,
        },
      ],
    },
    // Right sidebar
    {
      id: "right-sidebar",
      direction: WindowDirection.Vertical,
      display: WindowDisplay.Accordion,
      size: 256,
      children: [
        // Game notes
        {
          id: "game-notes",
          size: 256,
          resizable: true,
          collapsed: false,
        },
      ],
    },
  ],
};

const layout = ref<ILayout>(defaultLayout);

// Helper to find a window in the layout by its id
// searches recursively through all children
function findWindowInLayout(windowId: string): ILayout | null {
  const isContainer = validateWindowContainer(layout.value);
  if (!isContainer.success) {
    return null;
  }
  for (const child of (layout.value as IWindowContainer).children) {
    if (child.id === windowId) {
      return child;
    }
  }
  return null;
}

function collapseWindow(windowId: string) {
  console.log("Collapsing window:", windowId);
  // Find the window in the layout
  const window = findWindowInLayout(windowId);
  if (window && validateWindowContainer(window).success) {
    (window as IWindowContainer).collapsed = !window.collapsed;
  }
}
</script>

<template>
  <AppLayout
    :layout="layout"
    @update:layout="layout = $event"
    @update:toggle-collapse="collapseWindow"
  >
    <template #menu-bar>
      <Menubar :model="menuItems" v-model:visible="menuVisible">
        <template #start>
          <h1>Open Knight</h1>
        </template>

        <template #item="{ item, props, hasSubmenu, root }">
          <a v-ripple class="flex items-center" v-bind="props.action">
            <span>{{ item.label }}</span>
            <Badge
              v-if="item.badge"
              :class="{ 'ml-auto': !root, 'ml-2': root }"
              :value="item.badge"
            />
            <span
              v-if="item.shortcut"
              class="ml-auto border border-surface rounded bg-emphasis text-muted-color text-xs p-1"
              >{{ item.shortcut }}</span
            >
            <i
              v-if="hasSubmenu"
              :class="[
                'pi pi-angle-down ml-auto',
                { 'pi-angle-down': root, 'pi-angle-right': !root },
              ]"
            ></i>
          </a>
        </template>

        <template #end>
          <!-- Theme Toggle -->
          <div
            style="
              display: flex;
              gap: 10px;
              align-items: center;
              justify-content: center;
            "
          >
            <Button label="Toggle theme" @click="toggleTheme" />
            <Button
              label="Empty DB"
              @click="emptyDb"
              icon="pi pi-trash"
              severity="danger"
            />
          </div>
        </template>
      </Menubar>
    </template>

    <template #game-explorer>
      <GameExplorer
        :games="games"
        :selectedGame="selectedGame"
        @update:pgn="pgn = $event"
        @parse-pgn="parsePgn"
        @update:selectedGame="selectedGame = $event"
      />
    </template>

    <template #game-board>
      <GameBoard :selectedGame="selectedGame" />
    </template>

    <template #game-notes>
      <div class="flex flex-col">
        <h1>Game Notes</h1>
      </div>
    </template>
  </AppLayout>
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

html {
  color: var(--p-primary-color);
  background-color: var(--p-content-background);

  padding: 0;
  margin: 0;
  border: 0;

  width: 100%;
  height: 100%;
}
</style>
