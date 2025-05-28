<template>
  <div class="flex flex-col bg-base-200 rounded-lg">
    <!-- Header -->
    <div
      class="flex justify-between items-center p-3 border-b border-base-300 flex-shrink-0"
    >
      <div class="flex gap-2 items-center">
        <select v-model="selectedEngine" class="select select-sm w-auto">
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
          Load Engine
        </button>

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
      <div v-if="latestAnalysisResult" class="mb-4">
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
          <div v-if="latestBestMove" class="text-sm">
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
            <strong>Line:</strong> {{ formatPV(latestAnalysisResult?.pv) }}
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
          Move {{ gameAnalysisProgress.current }} of
          {{ gameAnalysisProgress.total }}
        </div>
      </div>

      <!-- New Engine Form -->
      <div
        v-if="selectedEngine === 'New Engine'"
        class="flex flex-col gap-2 mb-4"
      >
        <label for="engineName" class="input input-md flex items-center gap-4">
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

        <label for="enginePath" class="input input-md flex items-center gap-4">
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
        :engineSettings="engineSettings"
        @update:engineSettings="onEngineSettingsUpdate"
        class="flex-1 min-h-0"
      />
    </div>
  </div>

  <Teleport to="body">
    <div class="toast" v-if="toast">
      <div class="alert alert-info">
        <p>{{ engineMessage }}</p>
      </div>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { PhBinary, PhIdentificationCard } from "@phosphor-icons/vue";
import { Teleport, computed, onMounted, ref, watch } from "vue";
import type { AnalysisUpdate, EngineOption } from "../../shared/types";
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
const gameAnalysisProgress = ref({ current: 0, total: 0 });

const engineAnalysisStore = useEngineAnalysisStore();

const engineSettings = computed(() => {
  // convert the engine settings from a map to a list of key-value pairs
  return Object.entries(
    engineAnalysisStore.getEngineSettings(selectedEngine.value) ?? {}
  )
    .filter(
      ([key, value]) =>
        key !== undefined && value !== undefined && !key.includes("UCI")
    )
    .map(
      ([key, value]) => [key, value as EngineOption] as [string, EngineOption]
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

const engineMessage = ref<string>("");
const toast = ref<boolean>(false);

const isAnalyzing = computed(() => {
  const engine = engineAnalysisStore.engines.get(selectedEngine.value);
  return engine ? engine.isAnalyzing : false;
});

const isGameAnalysisInProgress = computed(
  () => engineAnalysisStore.gameAnalysisInProgress
);

const onStockfishAnalysisResult = (result: { [key: string]: unknown }) => {
  const messageType = result.message_type;
  if (messageType === "info") {
    // Get the info returned by the engine (the non-empty key-value pairs)
    const info = Object.fromEntries(
      Object.entries(result).filter(
        ([key, value]) =>
          key !== "message_type" &&
          value !== "" &&
          value !== null &&
          value !== undefined
      )
    );

    // Prettify the info object (into key: value format)
    const prettyInfo = Object.entries(info)
      .map(([key, value]) => `${key}: ${value}`)
      .join("\n");
    engineMessage.value = `Stockfish 17\n${prettyInfo}`;

    toast.value = true;
    setTimeout(() => {
      toast.value = false;
    }, 3000);
  }
};

// Watch for position changes to update analysis
watch(currentPosition, (newPosition) => {
  if (isAnalyzing.value && newPosition?.fen) {
    startAnalysis();
  }
});

onMounted(async () => {
  engineAnalysisStore.initAnalysisService();
  try {
    if (!engineAnalysisStore.engines.has(selectedEngine.value)) {
      engineAnalysisStore.addAnalysisListener(
        selectedEngine.value,
        onStockfishAnalysisResult as unknown as (result: AnalysisUpdate) => void
      );
    }
  } catch (error) {
    if (error instanceof Error && "EngineError" in error) {
      console.error("Engine error:", error.EngineError);
    } else {
      console.error("Failed to load engine:", error);
    }
  }
});

async function loadEngine() {
  await engineAnalysisStore.loadEngine(
    newEngineName.value,
    newEnginePath.value
  );
  if (!availableEngines.value.includes(newEngineName.value)) {
    availableEngines.value.push(newEngineName.value);
  }
  selectedEngine.value = newEngineName.value;
}

async function startAnalysis() {
  if (!currentPosition.value?.fen) return;
  try {
    await engineAnalysisStore.analyzePosition(
      selectedEngine.value,
      currentPosition.value.fen
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
      currentGame.value.id
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
  return pv.slice(0, 5).join(" ");
}

function onEngineSettingsUpdate(updatedSettings: [string, EngineOption][]) {
  // Update the engine settings in the store
  const settingsObj: Record<string, EngineOption> =
    Object.fromEntries(updatedSettings);
  engineAnalysisStore.setEngineSettings(selectedEngine.value, settingsObj);
}
</script>
