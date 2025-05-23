<script setup lang="ts">
import { computed, onMounted, provide, ref } from "vue";
import ChessBoard from "./components/ChessBoard/ChessBoard.vue";
import EngineAnalysisPanel from "./components/EngineAnalysis/EngineAnalysisPanel.vue";
import EvaluationBar from "./components/EvaluationBar/EvaluationBar.vue";
import GameLibrary from "./components/GameLibrary/GameLibrary.vue";
import ImportModal from "./components/ImportModal/ImportModal.vue";
import MoveTree from "./components/MoveTree/MoveTree.vue";
import Navbar from "./components/Navbar/Navbar.vue";
import SettingsModal from "./components/Settings/SettingsModal.vue";
import { useGlobalStore } from "./stores";

const importModalOpen = ref(false);

const globalStore = useGlobalStore();

const engineAnalysisStore = globalStore.engineAnalysisStore;

const uiStore = globalStore.uiStore;
const settingsModalOpen = computed(() => uiStore.getSettingsModalOpen);

const displayGameLibrary = computed(() => uiStore.getGameLibraryViewOpen);
const displayMoveTree = computed(() => uiStore.getMoveTreeViewOpen);
const displayEngineView = computed(() => uiStore.getEngineViewOpen);

const refreshGamesClick = async () => {
	await globalStore.fetchExplorerGames();
};

const resetDatabaseClick = async () => {
	await globalStore.resetDatabase();
};

const newGameClick = async () => {
	await globalStore.gamesStore.newGame(0);
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
provide("size", 25);
provide("weight", "bold");
provide("mirrored", false);
</script>

<template>
  <div class="flex flex-col h-screen w-screen">
    <Navbar
      v-model:importModalOpen="importModalOpen"
      @refreshGames="refreshGamesClick"
      @resetDatabase="resetDatabaseClick"
      @newGame="newGameClick"
    />

    <main
      class="grid grow bg-base-100 text-base-content"
      :class="{
        'grid-cols-8':
          (displayGameLibrary && displayMoveTree && !displayEngineView) ||
          (displayEngineView && !displayGameLibrary && !displayMoveTree),
        'grid-cols-6':
          (displayGameLibrary && !displayMoveTree && !displayEngineView) ||
          (!displayGameLibrary && displayMoveTree && !displayEngineView),
        'grid-cols-4':
          !displayGameLibrary && !displayMoveTree && !displayEngineView,
      }"
    >
      <!-- TODO: Remove this -->
      <div class="col-span-4 flex flex-col" v-if="displayEngineView">
        <EngineAnalysisPanel :board-id="0" />
      </div>
      <div class="col-span-2 flex flex-col" v-if="displayGameLibrary">
        <GameLibrary />
      </div>
      <div class="col-span-4 flex flex-col items-center justify-center">
        <!-- Game board -->
        <div class="flex flex-row">
          <EvaluationBar
            class="min-h-full max-w-10 float-left"
            :evaluation="engineAnalysisStore.boardEvaluation"
            orientation="black"
            direction="vertical"
          />
          <div class="flex flex-col">
            <ChessBoard :board-id="0" />
          </div>
        </div>
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
