<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
import Badge from "primevue/badge";
import Menubar from "primevue/menubar";
import { onMounted, ref } from "vue";
import GameBoard from "./components/GameBoard.vue";
import GameExplorer from "./components/GameExplorer.vue";
import {
  apiExplorerStateToExplorerState,
  apiSelectedGameToGame,
} from "./shared/api-conversions";
import { IGame } from "./shared/types";

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

async function setSelectedGame(game: IGame | null) {
  // Unselect
  if (game === selectedGame.value) {
    selectedGame.value = null;
    await invoke("set_selected_game", { gameId: null });
    return;
  }

  selectedGame.value = game;
  if (selectedGame.value) {
    console.log("Setting selected game:", selectedGame.value);
    await invoke("set_selected_game", { gameId: selectedGame.value.id });
  }
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
</script>

<template>
  <div class="container">
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

    <GameExplorer
      :games="games"
      :selectedGame="selectedGame"
      @update:pgn="pgn = $event"
      @parse-pgn="parsePgn"
      @update:selectedGame="setSelectedGame"
    />
    <GameBoard :selectedGame="selectedGame" />
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
}

html {
  color: var(--p-primary-color);
  background-color: var(--p-content-background);
}
</style>
