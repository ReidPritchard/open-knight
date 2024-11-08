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
import { useGlobalState } from "../shared/store";
import { IMove } from "../shared/types";
import ChessMoveTree from "./ChessMoveTree/ChessMoveTree.vue";

const { selectedGame, selectedGameLocation, setSelectedGameLocation } =
  useGlobalState();

const moves = computed<IMove[]>(() => {
  return selectedGame.value?.moves ?? [];
});

const currentMoveId = computed<number>(() => {
  return selectedGame.value?.moves?.[selectedGameLocation.value ?? 0]?.id ?? 0;
});

const handleMoveClick = (move: { move: IMove; index: number }) => {
  setSelectedGameLocation(move.index);
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
