<script setup lang="ts">
import { computed, onMounted } from "vue";
import { useGlobalStore } from "./stores";
import ChessBoard from "./components/ChessBoard/ChessBoard.vue";
import GameLibrary from "./components/GameLibrary/GameLibrary.vue";

const globalStore = useGlobalStore();

const uiStore = globalStore.uiStore;

const displayGameLibrary = computed(() => uiStore.getGameLibraryViewOpen);
const toggleGameLibraryView = () => {
  uiStore.toggleGameLibraryView();
};

const importDemoGamesClick = async () => {
  await globalStore.importDemoGames();
};

const refreshGamesClick = async () => {
  await globalStore.fetchExplorerGames();
};

const resetDatabaseClick = async () => {
  await globalStore.resetDatabase();
};

onMounted(() => {
  globalStore.fetchExplorerGames();
});
</script>

<template>
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
            class="menu menu-sm dropdown-content bg-base-100 rounded-box z-[1] mt-3 w-52 p-2 shadow"
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
              <button class="btn btn-ghost" @click="importDemoGamesClick">
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
                <span class="material-symbols-outlined"> delete </span>
              </button>
            </li>
          </ul>
        </div>
      </div>
      <div class="navbar-center">
        <a class="btn btn-ghost text-xl"> Open Knight </a>
      </div>
      <div class="navbar-end">
        <button class="btn btn-ghost btn-circle">
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
        <button class="btn btn-ghost btn-circle">
          <div class="indicator">
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
                d="M15 17h5l-1.405-1.405A2.032 2.032 0 0118 14.158V11a6.002 6.002 0 00-4-5.659V5a2 2 0 10-4 0v.341C7.67 6.165 6 8.388 6 11v3.159c0 .538-.214 1.055-.595 1.436L4 17h5m6 0v1a3 3 0 11-6 0v-1m6 0H9"
              />
            </svg>
            <span class="badge badge-xs badge-primary indicator-item"></span>
          </div>
        </button>
      </div>
    </div>
  </header>

  <main class="flex flex-row h-full w-full bg-base-100 text-base-content">
    <div class="flex flex-col">
      <!-- Game board -->
      <ChessBoard :board-id="0" />
    </div>
    <div class="flex flex-col" v-if="displayGameLibrary">
      <GameLibrary />
    </div>
  </main>
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
