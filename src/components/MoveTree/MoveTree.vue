<template>
  <div class="move-tree overflow-y-scroll max-h-[calc(100vh-5rem)]">
    <!-- Header -->
    <div class="flex items-center justify-between w-full bg-base-200 px-10">
      <h2
        class="text-lg font-bold sticky top-0 z-10 py-4 border-b border-base-300"
      >
        Move Tree
      </h2>
    </div>

    <ul class="steps steps-vertical w-full">
      <li
        v-for="move in moves"
        :key="move.value?.game_move?.ply_number"
        :data-content="move.value?.game_move?.ply_number ?? '0'"
        :data-ply-number="move.value?.game_move?.ply_number ?? 0"
        class="step cursor-pointer hover:bg-base-200 transition-colors duration-200 border-b-2 border-base-200 rounded-lg"
        :class="{
          // TODO: It would be cool to match white/black moves with the board's white/black square colors
          'step-primary':
            (move.value?.game_move?.ply_number ?? 0) <=
            (currentMove?.game_move?.ply_number ?? 0),
          'step-info': (move.value?.game_move?.ply_number ?? 0) % 2 === 1,
          'bg-base-200/50': (move.value?.game_move?.ply_number ?? 0) % 2 === 0,
          'bg-primary/50 hover:bg-primary/70':
            (move.value?.game_move?.ply_number ?? 0) ===
            (currentMove?.game_move?.ply_number ?? 0),
        }"
        @click="changeMove(move.value?.game_move?.id)"
      >
        <div class="flex items-center gap-2">
          <span class="text-md">
            {{ move.value?.game_move?.san }}
          </span>
        </div>
      </li>
    </ul>
  </div>
</template>

<script setup lang="ts">
import { computed, watch } from "vue";
import { useGlobalStore } from "../../stores";

const props = defineProps<{
  boardId: number;
}>();

const globalStore = useGlobalStore();
const gamesStore = globalStore.gamesStore;

const boardState = computed(() => gamesStore.getBoardState(props.boardId));

// Remove first node (which is a `null` node)
const moves = computed(() => boardState.value?.game.move_tree.nodes.slice(1));

// Use the new getter from the refactored store
const currentMove = computed(() => gamesStore.getCurrentMove(props.boardId));

watch(currentMove, (newVal) => {
  // Scroll to the current move
  const moveElement = document.querySelector(
    `.step[data-ply-number="${newVal?.game_move?.ply_number}"]`
  );
  if (moveElement) {
    moveElement.scrollIntoView({ behavior: "smooth", block: "center" });
  }
});

const changeMove = (moveId: number | undefined) => {
  if (!moveId) {
    console.error("No move ID provided");
    return;
  }

  // Find the move in the game tree to get its ply number
  const gameState = boardState.value;
  if (!gameState) return;

  const moveNode = gameState.game.move_tree.nodes.find(
    (node) => node.value?.game_move?.id === moveId
  );

  if (!moveNode?.value?.game_move?.ply_number) {
    console.error("Could not find move or ply number");
    return;
  }

  // Convert ply number to move number (0-based for the API)
  const moveNumber = moveNode.value.game_move.ply_number - 1;
  gamesStore.jumpToMove(props.boardId, moveNumber);
};
</script>
