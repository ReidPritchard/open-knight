<template>
  <div class="flex flex-col h-screen w-screen">
    <Navbar
      v-model:importModalOpen="importModalOpen"
      @refreshGames="refreshGamesClick"
      @resetDatabase="resetDatabaseClick"
    />

    <!-- Main Layout with Resizable Split Panes -->
    <main class="flex-1 flex">
      <!-- Left Panel (Game Library) -->
      <ResizablePanel
        v-if="displayGameLibrary"
        :initial-size="layout.leftPanelWidth"
        :min-size="200"
        :max-size="600"
        direction="horizontal"
        @resize="updateLeftPanelWidth"
        class="border-r border-base-300"
      >
        <GameLibrary />
      </ResizablePanel>

      <!-- Center Content Area -->
      <div class="flex-1 flex flex-col">
        <!-- Board Tabs (Multi-board support) -->
        <BoardTabs
          :boards="activeBoardIds"
          :active-board="activeBoardId"
          @create-board="createNewBoard"
          @switch-board="setActiveBoardId"
          @close-board="closeBoardTab"
          class="border-b border-base-300"
        />

        <!-- Board and Engine Split -->
        <div class="flex-1 flex">
          <div class="flex-1 flex flex-col items-center justify-center">
            <div class="flex flex-row">
              <EvaluationBar
                class="min-h-full max-w-10"
                :evaluation="engineAnalysisStore.boardEvaluation"
                orientation="black"
                direction="vertical"
              />
              <div class="flex flex-col">
                <ChessBoard :board-id="activeBoardId" />
              </div>
            </div>
          </div>

          <!-- Engine Analysis Panel -->
          <ResizablePanel
            v-if="displayEngineView"
            :initial-size="layout.enginePanelWidth"
            :min-size="300"
            :max-size="800"
            direction="horizontal"
            position="right"
            @resize="updateEnginePanelWidth"
            class="border-l border-base-300"
          >
            <EngineAnalysisPanel :board-id="activeBoardId" />
          </ResizablePanel>
        </div>
      </div>

      <!-- Right Panel (Move Tree / Analysis) -->
      <ResizablePanel
        v-if="displayMoveTree"
        :initial-size="layout.rightPanelWidth"
        :min-size="200"
        :max-size="500"
        direction="horizontal"
        position="right"
        @resize="updateRightPanelWidth"
        class="border-l border-base-300"
      >
        <MoveTree :board-id="activeBoardId" />
      </ResizablePanel>
    </main>
  </div>

  <!-- Modals -->
  <SettingsModal
    :is-open="settingsModalOpen"
    @close="uiStore.updateSettingsModalOpen(false)"
  />
  <ImportModal :is-open="importModalOpen" @close="importModalOpen = false" />
</template>

<script setup lang="ts">
import { computed, onMounted, provide, ref } from "vue";
import ChessBoard from "./components/ChessBoard/ChessBoard.vue";
import EngineAnalysisPanel from "./components/EngineAnalysis/EngineAnalysisPanel.vue";
import EvaluationBar from "./components/EvaluationBar/EvaluationBar.vue";
import GameLibrary from "./components/GameLibrary/GameLibrary.vue";
import ImportModal from "./components/ImportModal/ImportModal.vue";
import BoardTabs from "./components/Layout/BoardTabs/BoardTabs.vue";
import ResizablePanel from "./components/Layout/ResizablePanel/ResizablePanel.vue";
import MoveTree from "./components/MoveTree/MoveTree.vue";
import Navbar from "./components/Navbar/Navbar.vue";
import SettingsModal from "./components/Settings/SettingsModal.vue";
import { useGlobalStore } from "./stores";

const importModalOpen = ref(false);
const globalStore = useGlobalStore();
const engineAnalysisStore = globalStore.engineAnalysisStore;
const uiStore = globalStore.uiStore;

// Layout state
const layout = computed(() => uiStore.layout);
const settingsModalOpen = computed(() => uiStore.getSettingsModalOpen);
const displayGameLibrary = computed(() => uiStore.getGameLibraryViewOpen);
const displayMoveTree = computed(() => uiStore.getMoveTreeViewOpen);
const displayEngineView = computed(() => uiStore.getEngineViewOpen);

// Multi-board support
const activeBoardIds = computed(() => uiStore.getActiveBoardIds);
const activeBoardId = computed(() => uiStore.getActiveBoardId);

// Layout resize handlers
const updateLeftPanelWidth = (width: number) => {
  uiStore.updateLayoutDimension("leftPanelWidth", width);
};

const updateRightPanelWidth = (width: number) => {
  uiStore.updateLayoutDimension("rightPanelWidth", width);
};

const updateEnginePanelWidth = (width: number) => {
  uiStore.updateLayoutDimension("enginePanelWidth", width);
};

// Board management
const setActiveBoardId = (boardId: number) => {
  uiStore.setActiveBoardId(boardId);
};

const closeBoardTab = (boardId: number) => {
  uiStore.closeBoardTab(boardId);
  globalStore.gamesStore.closeGame(boardId);
};

const createNewBoard = () => {
  uiStore.createNewBoard();
};

// Existing handlers
const refreshGamesClick = async () => {
  await globalStore.fetchExplorerGames();
};

const resetDatabaseClick = async () => {
  await globalStore.resetDatabase();
};

onMounted(() => {
  globalStore.fetchExplorerGames();

  // Load saved layout preferences
  uiStore.loadLayoutPreferences();

  // Development mode exposure
  if (import.meta.env.DEV) {
    const globalWindow = window as unknown as {
      $$: { store: typeof globalStore; api: typeof globalStore.api };
    };
    globalWindow.$$ = { store: globalStore, api: globalStore.api };
  }
});

// Setup default styles for Phosphor icons
provide("color", "currentColor");
provide("size", 30);
provide("weight", "bold");
provide("mirrored", false);
</script>

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
