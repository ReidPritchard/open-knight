<template>

	<div class="flex flex-col bg-base-200 rounded-lg">

		<!-- Header -->

		<div
			class="flex justify-between items-center p-3 border-b border-base-300 flex-shrink-0"
		>

			<div class="flex gap-2 items-center flex-wrap">

				<select
					v-model="selectedEngine"
					class="select select-sm w-auto"
				>

					<option
						v-for="engine in availableEngines"
						:key="engine"
						:value="engine"
					>
						 {{ engine }}
					</option>

				</select>

				<!-- Load engine -->

				<button
					v-if="selectedEngine === 'New Engine'"
					class="btn btn-sm btn-primary"
					@click="loadEngine()"
				>
					 Load
				</button>

				<button
					v-else
					class="btn btn-sm btn-primary"
					@click="unloadEngine()"
				>
					 Unload
				</button>

				<!-- Depth input -->

				<input
					v-if="selectedEngine !== 'New Engine'"
					type="number"
					v-model="depth"
					class="input input-sm"
				/>

				<button
					v-if="selectedEngine !== 'New Engine'"
					class="btn btn-sm"
					@click="isAnalyzing ? stopAnalysis() : startAnalysis()"
					:class="{ 'btn-primary': !isAnalyzing, 'btn-warning': isAnalyzing }"
				>
					 {{ isAnalyzing ? "Stop" : "Analyze" }}
				</button>

				<button
					v-if="selectedEngine !== 'New Engine'"
					class="btn btn-sm btn-primary"
					@click="startGameAnalysis()"
					:disabled="isGameAnalysisInProgress"
				>
					 Analyze Game
				</button>

			</div>

		</div>

		<!-- Content -->

		<div class="flex-1 min-h-0 p-3 flex flex-col gap-2">

			<div
				v-if="latestAnalysisResult"
				class="mb-4"
			>

				<!-- Current analysis info -->

				<div class="flex justify-between mb-2 text-xs text-base-content/70">

					<div>Depth: {{ latestAnalysisResult?.depth || 0 }}</div>

					<div>Nodes: {{ formatNodes(latestAnalysisResult?.nodes || 0) }}</div>

				</div>

				<!-- Current score -->

				<div class="flex justify-between mb-2 text-xs text-base-content/70">

					<!-- use a progress bar as an evaluation bar -->

					<progress
						class="progress progress-primary w-full"
						:value="50 + (latestAnalysisResult?.score?.value || 0) / 100"
						:max="100"
					></progress>

					<span class="text-xs ml-2">
						 {{ (latestAnalysisResult?.score?.value || 0) / 100 }}
					</span>

				</div>

				<!-- Best move/line -->

				<!-- TODO: Add multipv support -->

				<div class="flex flex-col gap-2">

					<div
						v-if="latestBestMove"
						class="text-sm"
					>

						<strong>Best move:</strong>
						 {{ formatSAN(latestBestMove.move) }}
						<span v-if="latestBestMove.ponder">

							<strong>Ponder:</strong>
							 {{ formatSAN(latestBestMove.ponder) }}
						</span>

					</div>

					<div
						v-if="
							latestAnalysisResult?.pv && latestAnalysisResult?.pv.length > 0
						"
						class="text-sm"
					>

						<strong>Line:</strong>
						 {{ formatPV(latestAnalysisResult?.pv) }}
					</div>

				</div>

			</div>

			<div
				v-if="isGameAnalysisInProgress"
				class="my-4 p-3 bg-base-300 rounded-md"
			>

				<div class="flex items-center gap-2 mb-2">

					<span class="loading loading-spinner loading-sm"></span>

					<span class="text-sm font-medium">Analyzing Game...</span>

				</div>

				<progress
					class="progress progress-primary w-full"
					:value="gameAnalysisProgress.current"
					:max="gameAnalysisProgress.total"
				></progress>

				<div class="text-xs text-base-content/70 mt-1">
					 Move {{ gameAnalysisProgress.current }} of {{
						gameAnalysisProgress.total
					}}
				</div>

			</div>

			<!-- New Engine Form -->

			<div
				v-if="selectedEngine === 'New Engine'"
				class="flex flex-col gap-2 mb-4"
			>

				<label
					for="engineName"
					class="input input-md flex items-center gap-4"
				>

					<PhIdentificationCard class="opacity-50" />

					<input
						id="engineName"
						type="text"
						v-model="newEngineName"
						placeholder="Engine Name"
						list="engineNames"
						class="grow"
					/>

					<datalist id="engineNames">

						<option value="stockfish" />

						<option value="lc0" />

						<option value="shredder" />

					</datalist>

				</label>

				<label
					for="enginePath"
					class="input input-md flex items-center gap-4"
				>

					<PhBinary class="opacity-50" />

					<input
						id="enginePath"
						type="text"
						v-model="newEnginePath"
						placeholder="Engine Path"
						autocomplete="off"
						list="enginePaths"
						class="grow"
					/>

					<datalist id="enginePaths">

						<!-- TODO: Maybe use a file browser, for now just provide some common paths
             to make input easier -->

						<option value="/usr/bin/" />

						<option value="/usr/local/bin/" />

						<option value="/usr/local/bin/stockfish" />

					</datalist>

				</label>

			</div>

			<!-- Engine settings -->

			<EngineSettings
				v-if="selectedEngine !== 'New Engine'"
				:engine-settings="engineSettings"
				@update:engine-settings="onEngineSettingsUpdate"
				class="flex-1 min-h-0"
			/>

		</div>

	</div>

</template>

<script setup lang="ts">
import { PhBinary, PhIdentificationCard } from "@phosphor-icons/vue";
import { computed, onMounted, ref, watch } from "vue";
import type { EngineOption } from "../../shared/types";
import { useGlobalStore } from "../../stores";
import { useEngineAnalysisStore } from "../../stores/engineAnalysis";
import EngineSettings from "./EngineSettings.vue";

const props = defineProps<{
	boardId: number;
}>();

const globalStore = useGlobalStore();
const gamesStore = globalStore.gamesStore;

const selectedEngine = ref<string>("New Engine");
const availableEngines = ref<string[]>(["New Engine"]);
const newEngineName = ref<string>("");
const newEnginePath = ref<string>("");
const depth = ref<number>(20);
const gameAnalysisProgress = ref({ current: 0, total: 0 });

const engineAnalysisStore = useEngineAnalysisStore();

const engineSettings = computed(() => {
	// convert the engine settings from a map to a list of key-value pairs
	return Object.entries(
		engineAnalysisStore.getEngineSettings(selectedEngine.value) ?? {},
	)
		.filter(
			([key, value]) =>
				key !== undefined && value !== undefined && !key.includes("UCI"),
		)
		.map(
			([key, value]) => [key, value as EngineOption] as [string, EngineOption],
		)
		.sort((a, b) => a[0].localeCompare(b[0]));
});
const currentPosition = computed(() => {
	return gamesStore.getCurrentPosition(props.boardId);
});
const currentGame = computed(() => {
	return gamesStore.getBoardState(props.boardId)?.game;
});
const latestAnalysisResult = computed(() => {
	return engineAnalysisStore.getLatestAnalysisUpdate(selectedEngine.value);
});
const latestBestMove = computed(() => {
	return engineAnalysisStore.getLatestBestMove(selectedEngine.value);
});

const isAnalyzing = computed(() => {
	const engine = engineAnalysisStore.engines.get(selectedEngine.value);
	return engine ? engine.isAnalyzing : false;
});

const isGameAnalysisInProgress = computed(
	() => engineAnalysisStore.gameAnalysisInProgress,
);

// Local storage key for saved engines
const SAVED_ENGINES_KEY = "open-knight-saved-engines";

interface SavedEngine {
	name: string;
	path: string;
}

// Save engine configuration to localStorage
function saveEngineToStorage(name: string, path: string) {
	try {
		const savedEngines = getSavedEngines();
		// Check if engine already exists, if so update it
		const existingIndex = savedEngines.findIndex(
			(engine) => engine.name === name,
		);
		if (existingIndex >= 0) {
			savedEngines[existingIndex] = { name, path };
		} else {
			savedEngines.push({ name, path });
		}
		localStorage.setItem(SAVED_ENGINES_KEY, JSON.stringify(savedEngines));
	} catch (error) {
		console.error("Failed to save engine to localStorage:", error);
	}
}

// Load saved engines from localStorage
function getSavedEngines(): SavedEngine[] {
	try {
		const saved = localStorage.getItem(SAVED_ENGINES_KEY);
		return saved ? JSON.parse(saved) : [];
	} catch (error) {
		console.error("Failed to load saved engines from localStorage:", error);
		return [];
	}
}

// Load saved engines into availableEngines
function loadSavedEngines() {
	const savedEngines = getSavedEngines();
	for (const engine of savedEngines) {
		if (!availableEngines.value.includes(engine.name)) {
			availableEngines.value.push(engine.name);
		}
	}
}

// Get saved engine path by name
function getSavedEnginePath(name: string): string | undefined {
	const savedEngines = getSavedEngines();
	return savedEngines.find((engine) => engine.name === name)?.path;
}

// Watch for position changes to update analysis
watch(currentPosition, (newPosition) => {
	if (isAnalyzing.value && newPosition?.fen) {
		startAnalysis();
	}
});

// Watch for engine selection changes to auto-populate saved engine paths
watch(selectedEngine, (newEngine) => {
	if (newEngine !== "New Engine") {
		const savedPath = getSavedEnginePath(newEngine);
		if (savedPath) {
			newEnginePath.value = savedPath;
			newEngineName.value = newEngine;
		}
	}
});

onMounted(async () => {
	engineAnalysisStore.initAnalysisService();
	loadSavedEngines();
});

async function loadEngine() {
	await engineAnalysisStore.loadEngine(
		newEngineName.value,
		newEnginePath.value,
	);

	// Save engine configuration to localStorage
	saveEngineToStorage(newEngineName.value, newEnginePath.value);

	if (!availableEngines.value.includes(newEngineName.value)) {
		availableEngines.value.push(newEngineName.value);
	}
	selectedEngine.value = newEngineName.value;
}

async function unloadEngine() {
	await engineAnalysisStore.unloadEngine(selectedEngine.value);
	selectedEngine.value = "New Engine";
}

async function startAnalysis() {
	if (!currentPosition.value?.fen) return;
	try {
		await engineAnalysisStore.analyzePosition(
			selectedEngine.value,
			currentPosition.value.fen,
			depth.value,
		);
	} catch (error) {
		console.error("Analysis error:", error);
		engineAnalysisStore.setEngineAnalyzing(selectedEngine.value, false);
	}
}

async function stopAnalysis() {
	try {
		await engineAnalysisStore.stopAnalysis(selectedEngine.value);
	} catch (error) {
		console.error("Failed to stop analysis:", error);
	}
}

async function startGameAnalysis() {
	if (!currentGame.value) return;
	try {
		engineAnalysisStore.setGameAnalysisInProgress(true);
		gameAnalysisProgress.value = { current: 0, total: 100 };
		await engineAnalysisStore.analyzeGame(
			selectedEngine.value,
			currentGame.value.id,
		);
		engineAnalysisStore.setGameAnalysisInProgress(false);
	} catch (error) {
		console.error("Game analysis error:", error);
		engineAnalysisStore.setGameAnalysisInProgress(false);
	}
}

function formatNodes(nodes: number): string {
	if (nodes >= 1_000_000_000) {
		return `${(nodes / 1_000_000_000).toFixed(1)}B`;
	}
	if (nodes >= 1_000_000) {
		return `${(nodes / 1_000_000).toFixed(1)}M`;
	}
	if (nodes >= 1_000) {
		return `${(nodes / 1_000).toFixed(1)}K`;
	}
	return nodes.toString();
}

function formatSAN(uci: string): string {
	// Ideally, convert UCI to SAN
	// For now, just return the UCI
	return uci;
}

function formatPV(pv: string[]): string {
	// Ideally, convert each UCI move to SAN
	// For now, just join them
	return pv.join(" ");
}

function onEngineSettingsUpdate(updatedSettings: [string, EngineOption][]) {
	// Update the engine settings in the store
	const settingsObj: Record<string, EngineOption> =
		Object.fromEntries(updatedSettings);
	engineAnalysisStore.setEngineSettings(selectedEngine.value, settingsObj);
}

// TODO: At some point we should save them to the database, but that's a low priority for now.
// NOTE: Basic localStorage saving is now implemented for engine name and path.
</script>

