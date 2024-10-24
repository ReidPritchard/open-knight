<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core";
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
  games.value = parsedState.games;
}

async function getSelectedGame() {
  const game: string = await invoke("get_selected_game");
  const parsedGame = apiSelectedGameToGame(game);
  selectedGame.value = parsedGame;
}

async function parsePgn() {
  await invoke("parse_pgn", { pgn: pgn.value });
  await updateGames();
  await getSelectedGame();
}

async function setSelectedGame(game: IGame | null) {
  if (game === selectedGame.value) {
    // Unselect
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
        <Button label="Toggle theme" @click="toggleTheme" />
      </template>
    </Menubar>

    <GameExplorer
      :games="games"
      :selectedGame="selectedGame"
      :pgn="pgn"
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
