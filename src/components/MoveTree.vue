<template>
  <div class="move-tree">
    <ChessMoveTree
      :moves="moves"
      :current-move-id="currentMoveId"
      @move-click="handleMoveClick"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { APIMove } from "../shared/bindings/APIMove";
import { useGameStore } from "../stores/game";
import ChessMoveTree from "./ChessMoveTree/ChessMoveTree.vue";

const gameStore = useGameStore();

const moves = computed<APIMove[]>(() => {
  return gameStore.selectedGame?.moves ?? [];
});

const currentMoveId = computed<number>(() => {
  return gameStore.currentMove?.game_move.id ?? 0;
});

const handleMoveClick = (move: { move: APIMove; index: number }) => {
  gameStore.setSelectedGameLocation(move.index);
};
</script>

<style scoped>
.move-tree {
  background-color: var(--p-surface-color);
  padding: 0.5rem;
  max-height: 150px;
  overflow-y: scroll;
}
</style>
