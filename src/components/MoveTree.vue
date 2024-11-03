<template>
  <div class="move-tree">
    <ChessMoveTree :moves="moves" />
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useGlobalState } from "../shared/store";
import { IMove } from "../shared/types";
import ChessMoveTree from "./ChessMoveTree/ChessMoveTree.vue";

const { selectedGame } = useGlobalState();

const moves = computed<IMove[]>(() => {
  return selectedGame.value?.moves ?? [];
});
</script>

<style scoped>
.move-tree {
  background-color: var(--p-surface-color);
  padding: 0.5rem;

  max-height: 150px;

  overflow-y: scroll;
}
</style>
