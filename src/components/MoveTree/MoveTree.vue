<template>
  <div class="move-tree overflow-y-scroll max-h-[calc(100vh-5rem)] px-4">
    <h2
      class="text-lg font-bold sticky top-0 bg-base-100 z-10 py-4 border-b border-base-300"
    >
      Move Tree
    </h2>
    <ul class="steps steps-vertical w-full">
      <li
        v-for="move in moves"
        :key="move.id"
        :data-content="
          move.ply_number % 2 === 0
            ? move.ply_number / 2
            : (move.ply_number + 1) / 2
        "
        :data-ply-number="move.ply_number"
        class="step cursor-pointer hover:bg-base-200 transition-colors duration-200 border-b-2 border-base-200 rounded-lg"
        :class="{
          // TODO: It would be cool to match white/black moves with the board's white/black square colors
          'step-primary': move.ply_number <= (currentMove?.ply_number ?? 0),
          'step-info': move.ply_number % 2 === 1,
          'bg-base-200/50': move.ply_number % 2 === 0,
          'bg-primary/50 hover:bg-primary/70':
            move.ply_number === currentMove?.ply_number,
        }"
        @click="changeMove(move.id)"
      >
        <div class="flex items-center gap-2">
          <span class="text-md">
            {{ move.san }}
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

const moves = computed(() => boardState.value?.game.moves);
const currentMove = computed(() => boardState.value?.currentMove);

watch(currentMove, (newVal) => {
  // Scroll to the current move
  const moveElement = document.querySelector(
    `.step[data-ply-number="${newVal?.ply_number}"]`
  );
  if (moveElement) {
    moveElement.scrollIntoView({ behavior: "smooth", block: "center" });
  }
});

const changeMove = (moveId: number) => {
  gamesStore.jumpToMove(props.boardId, moveId);
};
</script>
