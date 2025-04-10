<script setup lang="ts">
import { listen } from "@tauri-apps/api/event";
import { computed, onMounted, provide, ref } from "vue";
import ChessBoard from "./components/ChessBoard/ChessBoard.vue";
import GameLibrary from "./components/GameLibrary/GameLibrary.vue";
import ImportModal from "./components/ImportModal/ImportModal.vue";
import MoveTree from "./components/MoveTree/MoveTree.vue";
import SettingsModal from "./components/Settings/SettingsModal.vue";
import { useGlobalStore } from "./stores";

const importModalOpen = ref(false);

listen("engine-output", (event: { event: string; payload: string }) => {
  // console.log("Engine output", event.payload);
  // ex. "option name Threads type spin default 1 min 1 max 1024"
  // ex. "bestmove e2e4 ponder e2e4"
  // ex. "info depth 1 seldepth 1 score cp 100 nodes 1000 nps 1000000 tbhits 0 time 1000000000000000000"

  // Use the first word to determine the type of message
  const messageType = event.payload.split(" ")[0];
  switch (messageType) {
    case "option": {
      // Parse the option line
      const option = event.payload.split(" ");
      console.log("Option", option);
      break;
    }
    case "bestmove": {
      // Parse the bestmove line
      const bestmove = event.payload.split(" ");
      console.log("Bestmove", bestmove);
      break;
    }
    case "info": {
      // Parse the info line
      const info = event.payload.split(" ");
      // Info is a list of key-value pairs (until `pv` is reached)
      const infoMap = new Map();
      for (let i = 1; i < info.length; i += 2) {
        if (info[i] === "pv") {
          // The next item(s) is the pv line
          const pv = info.slice(i + 1).join(" ");
          infoMap.set("pv", pv);
          break;
        }

        if (info[i] === "score") {
          // The score info is "score <type> <value>"
          const scoreType = info[i + 1];
          const scoreValue = info[i + 2];
          const score =
            scoreType === "cp"
              ? Number.parseInt(scoreValue) / 100
              : scoreType === "mate"
              ? Number.parseInt(scoreValue)
              : Number.parseInt(scoreValue);
          infoMap.set("score", score);
          i += 1; // Add one to skip the extra item
        } else {
          infoMap.set(info[i], info[i + 1]);
        }
      }
      console.log("Info", infoMap);
      break;
    }
    default:
      break;
  }
});

const globalStore = useGlobalStore();

const uiStore = globalStore.uiStore;
const settingsModalOpen = computed(() => uiStore.getSettingsModalOpen);

const displayGameLibrary = computed(() => uiStore.getGameLibraryViewOpen);
const displayMoveTree = computed(() => uiStore.getMoveTreeViewOpen);
const displayEngineView = computed(() => uiStore.getEngineViewOpen);
const toggleGameLibraryView = () => {
  uiStore.toggleGameLibraryView();
};

const toggleMoveTreeView = () => {
  uiStore.toggleMoveTreeView();
};

const toggleEngineView = () => {
  // FIXME: Show UI for engine view
  uiStore.toggleEngineView();

  // for now, just call the api to analyze the current position
  // call await $$.api.engines.POST.loadEngine("tockfish", "/usr/local/bin/stockfish") first :)
  globalStore.analyzeCurrentPosition("stockfish", 0);
};

const refreshGamesClick = async () => {
  await globalStore.fetchExplorerGames();
};

const resetDatabaseClick = async () => {
  await globalStore.resetDatabase();
};

onMounted(() => {
  globalStore.fetchExplorerGames();

  // If in development mode, expose the state and API to the window
  if (import.meta.env.DEV) {
    const globalWindow = window as unknown as {
      $$: {
        store: typeof globalStore;
        api: typeof globalStore.api;
      };
    };

    globalWindow.$$ = {
      store: globalStore,
      api: globalStore.api,
    };
  }
});

// Setup default styles for Phosphor icons
provide("color", "currentColor");
provide("size", 30);
provide("weight", "bold");
provide("mirrored", false);
</script>

<template>
  <div class="flex flex-col h-screen w-screen">
    <header>
      <div
        class="navbar bg-base-100 text-blue-900 dark:text-blue-100 dark:bg-blue-900"
      >
        <div class="navbar-start">
          <div class="dropdown">
            <div tabindex="0" role="button" class="btn btn-ghost btn-circle">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                class="h-5 w-5"
                fill="none"
                viewBox="0 0 24 24"
                stroke="currentColor"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M4 6h16M4 12h16M4 18h7"
                />
              </svg>
            </div>
            <ul
              tabindex="0"
              class="menu menu-sm dropdown-content bg-base-100 rounded-box z-[1000] mt-3 w-52 p-2 shadow"
            >
              <li>
                <button class="btn btn-ghost">
                  <span class="material-symbols-outlined"> home </span>
                </button>
              </li>
              <li>
                <button class="btn btn-ghost" @click="toggleGameLibraryView">
                  <span
                    class="material-symbols-outlined"
                    :class="{ 'text-primary': displayGameLibrary }"
                  >
                    explore
                  </span>
                </button>
              </li>
              <li>
                <button
                  class="btn btn-ghost"
                  @click="toggleMoveTreeView"
                  :class="{ 'text-primary': displayMoveTree }"
                >
                  move tree
                </button>
              </li>
              <li>
                <button class="btn btn-ghost" @click="importModalOpen = true">
                  <span class="material-symbols-outlined"> import </span>
                </button>
              </li>
              <li>
                <button class="btn btn-ghost" @click="refreshGamesClick">
                  <span class="material-symbols-outlined"> refresh </span>
                </button>
              </li>
              <li>
                <button class="btn btn-ghost" @click="resetDatabaseClick">
                  <span class="material-symbols-outlined">
                    reset database
                  </span>
                </button>
              </li>
              <li>
                <button
                  class="btn btn-ghost"
                  @click="uiStore.updateSettingsModalOpen(true)"
                >
                  <span class="material-symbols-outlined"> settings </span>
                </button>
              </li>
            </ul>
          </div>
        </div>
        <div class="navbar-center">
          <a class="btn btn-ghost text-xl"> Open Knight </a>
        </div>
        <div class="navbar-end">
          <button
            class="btn btn-ghost btn-circle"
            @click="toggleEngineView"
            :class="{ 'text-primary': displayEngineView }"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              fill="none"
              viewBox="0 0 24 24"
              stroke="currentColor"
            >
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
              />
            </svg>
          </button>
        </div>
      </div>
    </header>

    <main
      class="grid grow bg-base-100 text-base-content"
      :class="{
        'grid-cols-8': displayGameLibrary && displayMoveTree,
        'grid-cols-6':
          (displayGameLibrary && !displayMoveTree) ||
          (!displayGameLibrary && displayMoveTree),
        'grid-cols-4': !displayGameLibrary && !displayMoveTree,
      }"
    >
      <div class="col-span-2 flex flex-col" v-if="displayGameLibrary">
        <GameLibrary />
      </div>
      <div class="col-span-4 flex flex-col items-center justify-center">
        <!-- Game board -->
        <ChessBoard :board-id="0" />
      </div>
      <div class="col-span-2 flex flex-col w-full" v-if="displayMoveTree">
        <MoveTree :board-id="0" />
      </div>
    </main>
  </div>

  <!-- Settings Modal -->
  <SettingsModal
    :is-open="settingsModalOpen"
    @close="uiStore.updateSettingsModalOpen(false)"
  />

  <!-- Import Modal -->
  <ImportModal :is-open="importModalOpen" @close="importModalOpen = false" />
</template>

<style>
:root {
  font-family: "Noto Sans Mono", monospace;
  font-optical-sizing: auto;
  font-weight: 400;
  font-style: normal;
  font-variation-settings: "wdth" 100;

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
</style>
