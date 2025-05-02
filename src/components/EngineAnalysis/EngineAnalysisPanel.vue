<template>
  <div class="flex flex-col h-full rounded-lg overflow-hidden bg-base-200">
    <div class="flex justify-between items-center p-3 border-b border-base-300">
      <h3 class="m-0 text-base font-semibold">Engine Analysis</h3>
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

        <button
          class="btn btn-sm"
          @click="isAnalyzing ? stopAnalysis() : startAnalysis()"
          :class="{ 'btn-primary': !isAnalyzing, 'btn-warning': isAnalyzing }"
        >
          {{ isAnalyzing ? "Stop" : "Analyze" }}
        </button>

        <button
          class="btn btn-sm btn-primary"
          @click="startGameAnalysis()"
          :disabled="isGameAnalysisInProgress"
        >
          Analyze Game
        </button>
      </div>
    </div>

    <div class="flex flex-1 overflow-hidden">
      <div class="flex-1 p-3 overflow-y-auto">
        <div v-if="isAnalyzing || analysisResult" class="mb-4">
          <div class="flex justify-between mb-2 text-xs text-base-content/70">
            <div>Depth: {{ analysisResult?.depth || 0 }}</div>
            <div>Nodes: {{ formatNodes(analysisResult?.nodes || 0) }}</div>
          </div>

          <div class="flex flex-col gap-2">
            <div v-if="analysisResult?.bestMove" class="text-sm">
              <strong>Best move:</strong>
              {{ formatSAN(analysisResult.bestMove) }}
            </div>

            <div
              v-if="analysisResult?.pv && analysisResult.pv.length > 0"
              class="text-sm"
            >
              <strong>Line:</strong> {{ formatPV(analysisResult.pv) }}
            </div>
          </div>
        </div>

        <div
          v-if="isGameAnalysisInProgress"
          class="my-4 p-3 bg-base-300 rounded-md"
        >
          <p>
            Analyzing game: {{ gameAnalysisProgress.current }} /
            {{ gameAnalysisProgress.total }}
          </p>
          <progress
            class="progress progress-primary w-full"
            :value="gameAnalysisProgress.current"
            :max="gameAnalysisProgress.total"
          ></progress>
        </div>
      </div>
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
import { Teleport, computed, onMounted, onUnmounted, ref, watch } from "vue";
import engineAnalysisService, {
  type EngineOption,
  type AnalysisResult,
  type EngineSettings,
} from "../../services/Analysis";
import { useGlobalStore } from "../../stores";

const props = defineProps<{
  boardId: number;
}>();

const globalStore = useGlobalStore();
const gamesStore = globalStore.gamesStore;

const selectedEngine = ref<string>("stockfish");
const availableEngines = ref<string[]>(["stockfish"]);
const isAnalyzing = ref<boolean>(false);
const analysisResult = ref<AnalysisResult | null>(null);
const isGameAnalysisInProgress = ref<boolean>(false);
const gameAnalysisProgress = ref({ current: 0, total: 0 });

const engineSettings = ref<EngineSettings>({
  depth: {
    option_type: "Spin",
    default: "18",
    min: 1,
    max: 100,
    var: null,
    value: null,
  },
  multiPV: {
    option_type: "Spin",
    default: "3",
    min: 1,
    max: 100,
    var: null,
    value: null,
  },
  threads: {
    option_type: "Spin",
    default: "4",
    min: 1,
    max: 100,
    var: null,
    value: null,
  },
  hashSize: {
    option_type: "Spin",
    default: "128",
    min: 1,
    max: 100,
    var: null,
    value: null,
  },
});

const currentPosition = computed(() => {
  return gamesStore.getBoardState(props.boardId)?.currentPosition;
});

const currentGame = computed(() => {
  return gamesStore.getBoardState(props.boardId)?.game;
});

const engineMessage = ref<string>("");
const toast = ref<boolean>(false);

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
    analysisResult.value = info as unknown as AnalysisResult;

    // Prettify the info object (into key: value format)
    const prettyInfo = Object.entries(info)
      .map(([key, value]) => `${key}: ${value}`)
      .join("\n");
    engineMessage.value = `Stockfish 17\n${prettyInfo}`;

    toast.value = true;
    setTimeout(() => {
      toast.value = false;
    }, 3000);
  } else if (messageType === "option") {
    const option = result as unknown as EngineOption & { name: string };
    engineSettings.value[option.name] = option;
  }
};

// Watch for position changes to update analysis
watch(currentPosition, (newPosition) => {
  if (isAnalyzing.value && newPosition?.fen) {
    startAnalysis();
  }
});

onMounted(async () => {
  try {
    // Load the engine if not already loaded
    if (!engineAnalysisService.isEngineAnalyzing(selectedEngine.value)) {
      // In production, you'd want to get the path from settings or auto-detect
      await engineAnalysisService.loadEngine(
        "stockfish",
        "/usr/local/bin/stockfish"
      );
      engineAnalysisService.addAnalysisListener(
        "Stockfish 17",
        onStockfishAnalysisResult as unknown as (result: AnalysisResult) => void
      );

      // Get current settings
      const settings = engineAnalysisService.getEngineSettings(
        selectedEngine.value
      );
      if (settings) {
        engineSettings.value = { ...settings };
      }
    }
  } catch (error) {
    // check if the error contains the `EngineError` property
    // if it does, we can log that message for a more accurate error message
    if (error instanceof Error && "EngineError" in error) {
      console.error("Engine error:", error.EngineError);
    } else {
      console.error("Failed to load engine:", error);
    }
  }
});

onUnmounted(() => {
  engineAnalysisService.destroy();
});

async function startAnalysis() {
  if (!currentPosition.value?.fen) return;

  try {
    isAnalyzing.value = true;

    await engineAnalysisService.analyzePosition(
      selectedEngine.value,
      currentPosition.value.fen,
      (result) => {
        analysisResult.value = result;
      }
    );
  } catch (error) {
    console.error("Analysis error:", error);
    isAnalyzing.value = false;
  }
}

async function stopAnalysis() {
  try {
    await engineAnalysisService.stopAnalysis(selectedEngine.value);
    isAnalyzing.value = false;
  } catch (error) {
    console.error("Failed to stop analysis:", error);
  }
}

async function startGameAnalysis() {
  if (!currentGame.value) return;

  try {
    isGameAnalysisInProgress.value = true;
    gameAnalysisProgress.value = { current: 0, total: 100 };

    await engineAnalysisService.analyzeGame(
      selectedEngine.value,
      currentGame.value.id
    );

    isGameAnalysisInProgress.value = false;
  } catch (error) {
    console.error("Game analysis error:", error);
    isGameAnalysisInProgress.value = false;
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
</script>
