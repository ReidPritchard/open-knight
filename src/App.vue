<template>

	<div
		class="h-screen w-screen max-h-screen max-w-screen flex flex-col select-none"
	>

		<Navbar
			v-model:import-modal-open="importModalOpen"
			@new-game="newGameClick"
			@refresh-games="refreshGamesClick"
			@reset-database="resetDatabaseClick"
			@quit-app="exitAppClick"
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
						@toggle="toggleLeftPanelSection('gameLibrary')"
						:min-height="400"
					>

						<GameLibrary
							:games="explorerGames"
							:is-loading="isLoading"
							:error="error"
							@open-game="handleOpenGame"
							@delete-game="handleDeleteGame"
							@update-game-property="handleUpdateGameProperty"
							@refresh-games="refreshGamesClick"
						/>

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

							<EvaluationBar
								class="min-h-full max-w-10"
								:evaluation="
									analysis?.score ?? { value: 0, type: 'centipawns' }
								"
								:evaluation-side="
									gamesStore.getCurrentTurn(activeBoardId) ?? 'white'
								"
								:orientation="
									uiStore.whiteOnSide === 'bottom' ? 'white' : 'black'
								"
								direction="vertical"
							/>

							<div class="flex flex-col">

								<ChessBoard
									:board-id="activeBoardId"
									:theme="uiStore.boardTheme"
									:show-coordinates="uiStore.showCoordinates"
									:show-legal-moves="uiStore.showLegalMoves"
									:is-flipped="uiStore.whiteOnSide === 'top'"
									:position="gamesStore.getCurrentPosition(activeBoardId)"
									:move="currentMove?.game_move ?? null"
									:valid-moves="validMoves"
									@make-move="handleMakeMove"
									@rotate-board="uiStore.setWhiteOnSide()"
									@resize-board="uiStore.updateBoardSquareSize($event)"
								/>

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

						<MoveDisplay
							v-if="boardState?.game"
							:move-tree="boardState.game.move_tree"
							@select-move="handleMoveSelect"
							@navigate-start="gamesStore.navigateToStart(activeBoardId)"
							@navigate-end="gamesStore.navigateToEnd(activeBoardId)"
							@navigate-previous="gamesStore.previousMove(activeBoardId)"
							@navigate-next="gamesStore.nextMove(activeBoardId)"
						/>

						<div
							v-else
							class="flex flex-col items-center justify-center h-full"
						>

							<p class="text-base-content/60">No game selected</p>

						</div>

					</template>

					<template #engine>

						<EngineAnalysisPanel
							:board-id="activeBoardId"
							:current-position-fen="
								gamesStore.getCurrentPosition(activeBoardId)?.fen ?? null
							"
							:current-game-id="boardState?.game.id ?? null"
							:engine-settings="
								Object.entries(
									engineAnalysisStore.getEngineSettings(selectedEngine),
								)
							"
							:latest-analysis-result="analysis ?? null"
							:latest-best-move="bestMove ?? null"
							:is-analyzing="engineAnalysisStore.isAnalyzing(selectedEngine)"
							:is-game-analysis-in-progress="
								engineAnalysisStore.gameAnalysisInProgress
							"
							:available-engines="availableEngines"
							:selected-engine="selectedEngine"
							@update:selected-engine="selectedEngine = $event"
							@load-engine="handleLoadEngine"
							@unload-engine="engineAnalysisStore.unloadEngine($event)"
							@start-analysis="handleStartAnalysis"
							@stop-analysis="engineAnalysisStore.stopAnalysis(selectedEngine)"
							@start-game-analysis="handleStartGameAnalysis"
							@update:engine-settings="handleUpdateEngineSettings"
						/>

					</template>

				</StackedPanel>

			</ResizablePanel>

		</main>

	</div>

	<!-- Modals -->

	<SettingsModal
		:is-open="uiStore.settingsModalOpen"
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
import { computed, onMounted, provide, ref, watch } from "vue";
import ChessBoard from "./components/ChessBoard/ChessBoard.vue";
import EngineAnalysisPanel from "./components/EngineAnalysis/EngineAnalysisPanel.vue";
import EvaluationBar from "./components/EvaluationBar/EvaluationBar.vue";
import GameLibrary from "./components/GameLibrary/GameLibrary.vue";
import ImportModal from "./components/ImportModal/ImportModal.vue";
import BoardTabs from "./components/Layout/BoardTabs/BoardTabs.vue";
import ResizablePanel from "./components/Layout/ResizablePanel/ResizablePanel.vue";
import { StackedPanel, StackedSection } from "./components/Layout/StackedPanel";
import MoveDisplay from "./components/MoveDisplay/MoveDisplay.vue";
import Navbar from "./components/Navbar/Navbar.vue";
import SettingsModal from "./components/Settings/SettingsModal.vue";
import Toasts from "./components/Toast/Toasts.vue";
import { API } from "./shared/api";
import type { EngineSettings, ExplorerGame } from "./shared/types";
import type { LegalMove } from "./shared/bindings";
import { useEngineAnalysisStore } from "./stores/engineAnalysis";
import { useGamesStore } from "./stores/games";
import { useUIStore } from "./stores/ui";
import { resetDatabase } from "./services/ImportExportService";
import { info } from "@tauri-apps/plugin-log";
import { exit } from "@tauri-apps/plugin-process";

const gamesStore = useGamesStore();
const engineAnalysisStore = useEngineAnalysisStore();
const uiStore = useUIStore();

const importModalOpen = ref(false);
const isLoading = ref(false);
const error = ref<string | null>(null);
async function execute<T>(promise: Promise<T>): Promise<T | undefined> {
	isLoading.value = true;
	error.value = null;
	try {
		return await promise;
	} catch (e) {
		if (
			typeof e === "object" &&
			e !== null &&
			"message" in e &&
			typeof e.message === "string"
		) {
			error.value = e.message;
		}
	} finally {
		isLoading.value = false;
	}
}

const explorerGames = ref<ExplorerGame[]>([]);
async function refreshGamesClick() {
	const result = await execute(API.games.list());
	if (result) explorerGames.value = result;
}

onMounted(async () => {
	await refreshGamesClick();
	engineAnalysisStore.initAnalysisService();
});

// =================================================================================================
// Board State
// =================================================================================================

const activeBoardId = computed(() => uiStore.activeBoardId);
const activeBoardIds = computed(() => uiStore.activeBoardIds);
const boardState = computed(() =>
	gamesStore.getBoardState(activeBoardId.value),
);
const currentPosition = computed(() =>
	gamesStore.getCurrentPosition(activeBoardId.value),
);
const currentMove = computed(() =>
	gamesStore.getCurrentMove(activeBoardId.value),
);

const validMoves = ref<LegalMove[] | null>(null);
watch(currentPosition, async (newPosition) => {
	if (!newPosition) {
		validMoves.value = null;
		return;
	}
	validMoves.value = await gamesStore.getValidMoves(activeBoardId.value);
});

function handleMakeMove(move: LegalMove) {
	gamesStore.makeMove(activeBoardId.value, move.uci);
}

function handleMoveSelect(moveId: number) {
	gamesStore.jumpToMove(activeBoardId.value, moveId);
}

function newGameClick() {
	gamesStore.newGame(activeBoardId.value);
}

async function handleOpenGame(payload: { gameId: number; newBoard: boolean }) {
	await gamesStore.openGame(payload.gameId, activeBoardId.value);
}

async function handleDeleteGame(gameId: number) {
	const deleted = await gamesStore.deleteGame(gameId);
	if (deleted) await refreshGamesClick();
}

async function handleUpdateGameProperty(payload: {
	gameId: number;
	field: string;
	value: string;
}) {
	await API.games.update(payload.gameId, payload.field, payload.value);
	await refreshGamesClick();
}

async function resetDatabaseClick() {
	await resetDatabase();
	await refreshGamesClick();
	await gamesStore.newGame(activeBoardId.value);
}

async function exitAppClick() {
	info("Exiting application...");
	exit(0);
}

// =================================================================================================
// Board Tabs
// =================================================================================================

function createNewBoard() {
	uiStore.createNewBoard();
}

function setActiveBoardId(boardId: number) {
	uiStore.setActiveBoardId(boardId);
}

function closeBoardTab(boardId: number) {
	gamesStore.closeGame(boardId);
	uiStore.closeBoardTab(boardId);
}

function renameBoard(boardId: number, name: string) {
	uiStore.renameBoard(boardId, name);
}

async function saveBoard(boardId: number) {
	await gamesStore.saveGame(boardId);
}

// =================================================================================================
// Panel Layout
// =================================================================================================

const layout = computed(() => uiStore.layout);
const displayLeftPanel = computed(() => uiStore.leftPanelOpen);
const displayRightPanel = computed(() => uiStore.rightPanelOpen);
const isLeftPanelSectionCollapsed = (section: string) =>
	uiStore
		.getStackedPanelState("leftPanel")
		?.collapsedSections?.includes(section) ?? false;
const rightPanelActiveTab = computed(
	() => uiStore.getStackedPanelState("rightPanel")?.activeTab,
);

const rightPanelSections = [
	{ id: "moveTree", title: "Move Tree", icon: PhTree },
	{ id: "engine", title: "Engine", icon: PhEngine },
];

function updateLeftPanelWidth(width: number) {
	uiStore.updateLayoutDimension("leftPanelWidth", width);
}

function updateRightPanelWidth(width: number) {
	uiStore.updateLayoutDimension("rightPanelWidth", width);
}

function toggleLeftPanelSection(sectionId: string) {
	uiStore.toggleStackedPanelSection("leftPanel", sectionId);
}

function handleLeftPanelSectionToggle(sectionId: string, collapsed: boolean) {
	if (
		(isLeftPanelSectionCollapsed(sectionId) && !collapsed) ||
		(!isLeftPanelSectionCollapsed(sectionId) && collapsed)
	) {
		uiStore.toggleStackedPanelSection("leftPanel", sectionId);
	}
}

function handleRightPanelTabChange(tabId: string) {
	uiStore.setStackedPanelActiveTab("rightPanel", tabId);
}

// =================================================================================================
// Engine Analysis
// =================================================================================================

const availableEngines = computed(() =>
	Array.from(engineAnalysisStore.engines.keys()),
);
const selectedEngine = ref("");
const analysis = computed(() =>
	engineAnalysisStore.getLatestAnalysisUpdate(selectedEngine.value),
);
const bestMove = computed(() =>
	engineAnalysisStore.getLatestBestMove(selectedEngine.value),
);

function handleUpdateEngineSettings(payload: {
	engine: string;
	settings: EngineSettings;
}) {
	engineAnalysisStore.updateEngineSettings(payload.engine, payload.settings);
}

async function handleLoadEngine(payload: { name: string; path: string }) {
	await engineAnalysisStore.loadEngine(payload.name, payload.path);

	// Switch to the new engine (if available)
	if (availableEngines.value.includes(payload.name)) {
		selectedEngine.value = payload.name;
	}
}

async function handleStartAnalysis(payload: {
	fen: string;
	depth: number;
	engine: string;
}) {
	const { fen, depth, engine } = payload;
	await engineAnalysisStore.analyzePosition(engine, fen, depth);
}

async function handleStartGameAnalysis() {
	info("Starting game analysis");
	const boardId = activeBoardId.value;
	if (boardId) {
		await engineAnalysisStore.analyzeGame(boardId);
	}
}

provide("color", "currentColor");
provide("size", 20);
provide("weight", "fill");
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

