<template>

	<div
		class="h-screen w-screen max-h-screen max-w-screen flex flex-col select-none"
	>

		<Navbar
			v-model:importModalOpen="importModalOpen"
			@newGame="newGameClick"
			@refreshGames="refreshGamesClick"
			@resetDatabase="resetDatabaseClick"
		/>

		<!-- Main Layout with Resizable Split Panes -->

		<main class="flex-1 min-h-0 min-w-0 overflow-auto flex flex-row">

			<!-- Left Panel (Game Library) -->

			<ResizablePanel
				v-if="displayLeftPanel"
				:initial-size="layout.leftPanelWidth"
				:min-size="200"
				:max-size="600"
				direction="horizontal"
				@resize="updateLeftPanelWidth"
				class="border-r border-base-300"
			>

				<StackedPanel
					name="leftPanel"
					mode="accordion"
					@section-toggle="handleLeftPanelSectionToggle"
				>

					<StackedSection
						title="Game Library"
						:icon="PhBooks"
						:collapsed="isLeftPanelSectionCollapsed('gameLibrary')"
						@toggle="toggleLeftPanelSection('gameLibrary', $event)"
						:min-height="400"
					>

						<GameLibrary />

					</StackedSection>

				</StackedPanel>

			</ResizablePanel>

			<!-- Center Content Area -->

			<div class="flex-1 flex flex-col">

				<!-- Board Tabs (Multi-board support) -->

				<BoardTabs
					:boards="activeBoardIds"
					:active-board="activeBoardId"
					:board-metadata="uiStore.getActiveBoardMetadata"
					@create-board="createNewBoard"
					@switch-board="setActiveBoardId"
					@close-board="closeBoardTab"
					@rename-board="renameBoard"
					@save-board="saveBoard"
					class="border-b border-base-300"
				/>

				<!-- Board and Engine Split -->

				<div class="flex-1 flex">

					<div class="flex-1 flex flex-col items-center justify-center">

						<div class="flex flex-row">

							<!-- TODO: We need to invalidate the evaluation when the board changes
							 currently it re-uses the evaluation from the previous position
							  -->

							<EvaluationBar
								class="min-h-full max-w-10"
								:evaluation="engineAnalysisStore.boardEvaluation"
								:evaluation-side="
									globalStore.gamesStore.getCurrentTurn(activeBoardId) ??
									'white'
								"
								:orientation="uiStore.whiteOnSide === 'top' ? 'black' : 'white'"
								direction="vertical"
							/>

							<div class="flex flex-col">

								<ChessBoard :board-id="activeBoardId" />

							</div>

						</div>

					</div>

				</div>

			</div>

			<!-- Right Panel (Move Tree / Analysis) -->

			<ResizablePanel
				v-if="displayRightPanel"
				:initial-size="layout.rightPanelWidth"
				:min-size="200"
				:max-size="500"
				direction="horizontal"
				position="right"
				@resize="updateRightPanelWidth"
				class="border-l border-base-300"
			>

				<StackedPanel
					name="rightPanel"
					mode="tabs"
					:sections="rightPanelSections"
					:active-tab="rightPanelActiveTab"
					@tab-change="handleRightPanelTabChange"
					persist-state
					storage-key="right-panel-tabs"
				>

					<template #moveTree>

						<MoveTree
							v-if="globalStore?.activeGame !== null"
							:moveTree="globalStore.activeGame.move_tree"
							@select-move="handleMoveSelect"
							@navigate-start="navigateToStart"
							@navigate-end="navigateToEnd"
							@navigate-previous="navigateToPrevious"
							@navigate-next="navigateToNext"
						/>

						<div
							v-else
							class="flex flex-col items-center justify-center h-full"
						>

							<p class="text-base-content/60">No game selected</p>

						</div>

					</template>

					<template #engine>

						<EngineAnalysisPanel :board-id="activeBoardId" />

					</template>

				</StackedPanel>

			</ResizablePanel>

		</main>

	</div>

	<!-- Modals -->

	<SettingsModal
		:is-open="settingsModalOpen"
		@close="uiStore.updateSettingsModalOpen(false)"
	/>

	<ImportModal
		:is-open="importModalOpen"
		@close="importModalOpen = false"
	/>

	<Toasts />

</template>

<script setup lang="ts">
import { PhBooks, PhEngine, PhTree } from "@phosphor-icons/vue";
import { computed, onMounted, provide, ref } from "vue";
import ChessBoard from "./components/ChessBoard/ChessBoard.vue";
import EngineAnalysisPanel from "./components/EngineAnalysis/EngineAnalysisPanel.vue";
import EvaluationBar from "./components/EvaluationBar/EvaluationBar.vue";
import GameLibrary from "./components/GameLibrary/GameLibrary.vue";
import ImportModal from "./components/ImportModal/ImportModal.vue";
import BoardTabs from "./components/Layout/BoardTabs/BoardTabs.vue";
import ResizablePanel from "./components/Layout/ResizablePanel/ResizablePanel.vue";
import { StackedPanel, StackedSection } from "./components/Layout/StackedPanel";
import MoveTree from "./components/MoveTree/MoveTree.vue";
import Navbar from "./components/Navbar/Navbar.vue";
import SettingsModal from "./components/Settings/SettingsModal.vue";
import Toasts from "./components/Toast/Toasts.vue";
import { useGlobalStore } from "./stores";

const importModalOpen = ref(false);
const globalStore = useGlobalStore();
const engineAnalysisStore = globalStore.engineAnalysisStore;
const uiStore = globalStore.uiStore;

// Layout state
const layout = computed(() => uiStore.layout);
const settingsModalOpen = computed(() => uiStore.getSettingsModalOpen);
const displayLeftPanel = computed(() => uiStore.getLeftPanelOpen);
const displayRightPanel = computed(() => uiStore.getRightPanelOpen);

// Multi-board support
const activeBoardIds = computed(() => uiStore.getActiveBoardIds);
const activeBoardId = computed(() => uiStore.getActiveBoardId);

// Right panel tabs configuration
const rightPanelSections = computed(() => [
	{
		id: "moveTree",
		title: "Move Tree",
		icon: PhTree,
	},
	{
		id: "engine",
		title: "Engine",
		icon: PhEngine,
	},
]);

const rightPanelActiveTab = computed(() => {
	const panelState = uiStore.getStackedPanelState("rightPanel");
	return panelState.activeTab || "moveTree";
});

// Layout resize handlers
const updateLeftPanelWidth = (width: number) => {
	uiStore.updateLayoutDimension("leftPanelWidth", width);
};

const updateRightPanelWidth = (width: number) => {
	uiStore.updateLayoutDimension("rightPanelWidth", width);
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

const renameBoard = (boardId: number, newName: string) => {
	uiStore.renameBoard(boardId, newName);
};

const saveBoard = (boardId: number) => {
	globalStore.gamesStore.saveGame(boardId);
	uiStore.updateBoardMetadata(boardId, { hasUnsavedChanges: false });
};

// Existing handlers
const refreshGamesClick = async () => {
	await globalStore.fetchExplorerGames();
};

const resetDatabaseClick = async () => {
	await globalStore.resetDatabase();
};

// Stacked panel handlers
const handleLeftPanelSectionToggle = (
	sectionId: string,
	_collapsed: boolean,
) => {
	uiStore.toggleStackedPanelSection("leftPanel", sectionId);
};

const toggleLeftPanelSection = (sectionId: string, _collapsed: boolean) => {
	uiStore.toggleStackedPanelSection("leftPanel", sectionId);
};

const isLeftPanelSectionCollapsed = (sectionId: string) => {
	const panelState = uiStore.getStackedPanelState("leftPanel");
	return panelState.collapsedSections?.includes(sectionId) ?? false;
};

const handleRightPanelTabChange = (tabId: string) => {
	uiStore.setStackedPanelActiveTab("rightPanel", tabId);
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

// Navigation event handlers for MoveTree
const handleMoveSelect = async (moveId: number) => {
	await globalStore.gamesStore.jumpToMove(activeBoardId.value, moveId);
};

const navigateToStart = async () => {
	await globalStore.gamesStore.navigateToStart(activeBoardId.value);
};

const navigateToEnd = async () => {
	await globalStore.gamesStore.navigateToEnd(activeBoardId.value);
};

const navigateToPrevious = async () => {
	await globalStore.gamesStore.previousMove(activeBoardId.value);
};

const navigateToNext = async () => {
	await globalStore.gamesStore.nextMove(activeBoardId.value);
};

const newGameClick = async () => {
	// TODO: Add UI for selecting variant

	const boardId = uiStore.createNewBoard();
	await globalStore.gamesStore.newGame(boardId, "standard");

	console.log("New game created:", globalStore.activeGame);
};
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

