<template>
  <div class="p-4 bg-base-100 rounded-lg flex flex-col min-h-0">
    <!-- Navigation buttons -->
    <div class="mb-4 flex justify-center">
      <div class="flex gap-2">
        <button
          @click="$emit('navigate-start')"
          class="btn btn-sm btn-ghost"
          :disabled="!currentNodeId"
        >
          <PhCaretLineLeft :size="16" />
        </button>
        <button
          @click="$emit('navigate-previous')"
          class="btn btn-sm btn-ghost"
          :disabled="!canNavigateBack"
        >
          <PhCaretLeft :size="16" />
        </button>
        <button
          @click="$emit('navigate-next')"
          class="btn btn-sm btn-ghost"
          :disabled="!canNavigateForward"
        >
          <PhCaretRight :size="16" />
        </button>
        <button
          @click="$emit('navigate-end')"
          class="btn btn-sm btn-ghost"
          :disabled="!hasMainLine"
        >
          <PhCaretLineRight :size="16" />
        </button>
      </div>
    </div>

    <!-- View mode toggle -->
    <div class="flex justify-center mb-2">
      <div class="join">
        <button
          v-for="mode in ['compact', 'tabular']"
          :key="mode"
          @click="viewMode = mode"
          class="join-item btn btn-xs"
          :class="{ 'btn-active': viewMode === mode }"
        >
          {{ mode }}
        </button>
      </div>
    </div>

    <!-- Current position info -->
    <div v-if="currentNode" class="my-4 space-y-2">
      <!-- Current move -->
      <div
        class="flex justify-between gap-2 text-md w-full border-b border-base-300 pb-2"
      >
        <PhCaretRight :size="16" />
        <span class="font-mono cursor-pointer">
          {{ currentMove?.san ?? "Initial position" }}
        </span>
        <PhCaretLeft :size="16" />
      </div>

      <!-- Evaluation display -->
      <div v-if="currentEvaluation" class="flex items-center gap-2 text-sm">
        <PhChartLine :size="16" />
        <span class="font-mono">
          {{ formatEvaluation(currentEvaluation) }}
        </span>
        <span class="text-base-content/60">
          depth {{ currentEvaluation.depth }}
        </span>
      </div>
      <div v-else class="flex items-center gap-2 text-sm">
        <span class="text-base-content/60">No evaluation</span>
      </div>

      <!-- Time info -->
      <div
        v-if="currentMove?.time_info"
        class="flex items-center gap-2 text-sm"
      >
        <PhClock :size="16" />
        <span>
          {{ formatTime(currentMove.time_info.time_spent_ms) }} spent
        </span>
        <span
          v-if="currentMove.time_info.time_left_ms"
          class="text-base-content/60"
        >
          ({{ formatTime(currentMove.time_info.time_left_ms) }} left)
        </span>
      </div>
      <div v-else class="flex items-center gap-2 text-sm">
        <span class="text-base-content/60">No time info</span>
      </div>

      <!-- Annotations -->
      <div v-if="currentAnnotations.length > 0" class="space-y-2">
        <div
          v-for="annotation in currentAnnotations"
          :key="annotation.id"
          class="bg-base-300 rounded p-2 text-sm"
        >
          <PhChatText :size="14" class="inline mr-1" />
          {{ annotation.comment }}
        </div>
      </div>
      <div v-else class="flex items-center gap-2 text-sm">
        <span class="text-base-content/60">No annotations</span>
      </div>
    </div>

    <!-- Move display area -->
    <div class="bg-base-200 rounded-lg p-4 flex-1 min-h-0" ref="moveContainer">
      <div v-if="rootNode">
        <!-- Compact view (optimized for long games) -->
        <div v-if="viewMode === 'compact'" class="space-y-2">
          <div
            v-for="(chunk, index) in moveChunks"
            :key="index"
            class="flex flex-wrap gap-1"
          >
            <button
              v-for="moveData in chunk"
              :key="moveData.nodeId.idx"
              @click="handleMoveSelect(moveData.nodeId)"
              class="px-2 py-1 rounded text-sm font-mono transition-colors cursor-pointer"
              :class="{
                'bg-primary text-primary-content': isCurrentMove(
                  moveData.nodeId
                ),
                'hover:bg-base-300': !isCurrentMove(moveData.nodeId),
                'opacity-60': moveData.isVariation,
              }"
            >
              <span v-if="moveData.showNumber" class="font-bold">
                {{ moveData.moveNumber }}.
              </span>
              {{ moveData.san }}
            </button>
          </div>
        </div>

        <!-- Tabular view -->
        <div v-if="viewMode === 'tabular'" class="overflow-x-auto">
          <table class="table table-xs">
            <tbody>
              <tr v-for="(row, index) in moveRows" :key="index">
                <td class="font-bold text-right">{{ row.number }}.</td>
                <td>
                  <button
                    v-if="row.white"
                    @click="handleMoveSelect(row.white.nodeId)"
                    class="px-2 py-1 rounded text-sm font-mono transition-colors cursor-pointer w-full text-left"
                    :class="{
                      'bg-primary text-primary-content': isCurrentMove(
                        row.white.nodeId
                      ),
                      'hover:bg-base-300': !isCurrentMove(row.white.nodeId),
                    }"
                  >
                    {{ row.white.san }}
                  </button>
                </td>
                <td>
                  <button
                    v-if="row.black"
                    @click="handleMoveSelect(row.black.nodeId)"
                    class="px-2 py-1 rounded text-sm font-mono transition-colors cursor-pointer w-full text-left"
                    :class="{
                      'bg-primary text-primary-content': isCurrentMove(
                        row.black.nodeId
                      ),
                      'hover:bg-base-300': !isCurrentMove(row.black.nodeId),
                    }"
                  >
                    {{ row.black.san }}
                  </button>
                </td>
              </tr>
            </tbody>
          </table>
        </div>
      </div>
      <div v-else class="text-center text-base-content/60 py-8">
        No moves to display
      </div>
    </div>
  </div>
</template>

<script setup>
import {
	PhCaretLeft,
	PhCaretLineLeft,
	PhCaretLineRight,
	PhCaretRight,
	PhChartLine,
	PhChatText,
	PhClock,
} from "@phosphor-icons/vue";
import { computed, nextTick, ref, watch } from "vue";

const props = defineProps({
	moveTree: {
		type: Object,
		required: true,
	},
});

const emit = defineEmits([
	"select-move",
	"navigate-start",
	"navigate-end",
	"navigate-previous",
	"navigate-next",
]);

// State
const viewMode = ref("compact"); // 'compact', 'tabular'
const moveContainer = ref(null);

// Computed properties
const rootNode = computed(() => {
	if (!props.moveTree.root_id) return null;
	return props.moveTree.nodes[props.moveTree.root_id.idx]?.value;
});

const currentNodeId = computed(() => props.moveTree.current_node_id);

const currentNode = computed(() => {
	if (!currentNodeId.value) return null;
	return props.moveTree.nodes[currentNodeId.value.idx]?.value;
});

const currentMove = computed(() => currentNode.value?.game_move);

const currentPosition = computed(() => currentNode.value?.position);

const currentEvaluation = computed(() => {
	if (!currentPosition.value?.evaluations?.length) return null;
	return currentPosition.value.evaluations.reduce((best, evaluation) => {
		if (!best || (evaluation.depth && evaluation.depth > (best.depth || 0)))
			return evaluation;
		return best;
	}, null);
});

const currentAnnotations = computed(() => {
	if (!currentMove.value?.annotations) return [];
	return currentMove.value.annotations.filter((a) => a.comment);
});

const canNavigateBack = computed(() => {
	return currentNode.value?.parent_id != null;
});

const canNavigateForward = computed(() => {
	return currentNode.value?.children_ids?.length > 0;
});

const hasMainLine = computed(() => {
	return props.moveTree.nodes.some((n) => n.value?.game_move);
});

// Flatten move tree for compact/tabular views
const flattenedMoves = computed(() => {
	if (!rootNode.value) return [];

	const moves = [];
	const visited = new Set();

	function traverse(node, isMainLine = true, depth = 0) {
		const nodeWrapper = props.moveTree.nodes.find((n) => n.value === node);
		if (!nodeWrapper) return;

		const nodeId = {
			idx: props.moveTree.nodes.indexOf(nodeWrapper),
			version: nodeWrapper.version,
		};

		// Avoid cycles
		const nodeKey = `${nodeId.idx}-${nodeId.version}`;
		if (visited.has(nodeKey)) return;
		visited.add(nodeKey);

		if (node.game_move) {
			moves.push({
				nodeId,
				node,
				move: node.game_move,
				san: node.game_move.san,
				plyNumber: node.game_move.ply_number,
				moveNumber: Math.ceil(node.game_move.ply_number / 2),
				showNumber: node.game_move.ply_number % 2 === 1,
				isWhite: node.game_move.ply_number % 2 === 1,
				isMainLine,
				isVariation: !isMainLine,
				depth,
			});
		}

		// Process children
		const children = node.children_ids
			.map((childId) => props.moveTree.nodes[childId.idx]?.value)
			.filter(Boolean);

		// First child continues main line
		if (children.length > 0) {
			traverse(children[0], isMainLine, depth);

			// Other children are variations
			for (let i = 1; i < children.length; i++) {
				traverse(children[i], false, depth + 1);
			}
		}
	}

	traverse(rootNode.value);
	return moves;
});

// Chunk moves for compact view (10 moves per line)
const moveChunks = computed(() => {
	const mainLineMoves = flattenedMoves.value.filter((m) => m.isMainLine);
	const chunks = [];
	const movesPerChunk = 10;

	for (let i = 0; i < mainLineMoves.length; i += movesPerChunk) {
		chunks.push(mainLineMoves.slice(i, i + movesPerChunk));
	}

	return chunks;
});

// Group moves by pairs for tabular view
const moveRows = computed(() => {
	const mainLineMoves = flattenedMoves.value.filter((m) => m.isMainLine);
	const rows = [];
	let currentRow = null;

	for (const move of mainLineMoves) {
		if (move.isWhite) {
			currentRow = { number: move.moveNumber, white: move, black: null };
			rows.push(currentRow);
		} else if (currentRow) {
			currentRow.black = move;
		}
	}

	return rows;
});

// Methods
const handleMoveSelect = (nodeId) => {
	emit("select-move", nodeId);
};

const isCurrentMove = (nodeId) => {
	return (
		currentNodeId.value &&
		currentNodeId.value.idx === nodeId.idx &&
		currentNodeId.value.version === nodeId.version
	);
};

const formatEvaluation = (evaluation) => {
	if (!evaluation || evaluation.score == null) return "?";
	const score = evaluation.score / 100;
	if (evaluation.eval_type === "mate") {
		return `#${evaluation.score > 0 ? "+" : ""}${evaluation.score}`;
	}
	return `${score > 0 ? "+" : ""}${score.toFixed(2)}`;
};

const formatTime = (ms) => {
	if (!ms) return "0:00";
	const seconds = Math.floor(ms / 1000);
	const minutes = Math.floor(seconds / 60);
	const remainingSeconds = seconds % 60;
	return `${minutes}:${remainingSeconds.toString().padStart(2, "0")}`;
};

// Auto-scroll to current move
watch(currentNodeId, async () => {
	await nextTick();
	if (moveContainer.value) {
		const currentButton = moveContainer.value.querySelector(".bg-primary");
		if (currentButton) {
			currentButton.scrollIntoView({ behavior: "smooth", block: "center" });
		}
	}
});
</script>
