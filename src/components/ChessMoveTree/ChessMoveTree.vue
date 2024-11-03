<template>
  <div class="move-tree">
    <div v-for="(move, index) in moveTree" class="move-tree-item">
      <ChessMove :key="move.id" :move="move" @move-click="handleMoveClick" />
      <div v-if="move.variations.length > 0" class="move-tree-item-variations">
        <ChessMoveTree :moves="move.variations" />
      </div>
      <!-- If there is a move after this move, add a separator -->
      <div
        v-if="move.nextMoveId"
        class="move-tree-item-separator"
        :class="{ 'turn-separator': index % 2 === 1 }"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { IMove } from "../../shared/types";
import ChessMove from "./ChessMove.vue";

const props = defineProps<{
  moves: IMove[];
}>();

const buildMoveTree = (moves: IMove[]) => {
  const moveTree: Array<
    IMove & { variations: IMove[]; nextMoveId: number | undefined }
  > = [];

  let nextMoveId: number | undefined = moves[1]?.id;
  moves.forEach((move, index) => {
    nextMoveId = moves[index + 1]?.id;
    const newMove = { ...move, variations: [], nextMoveId };
    moveTree.push(newMove);
  });

  return moveTree;
};

const moveTree = computed(() => buildMoveTree(props.moves));

const handleMoveClick = (move: IMove) => {
  console.log(move);
};
</script>

<style scoped>
* {
  --move-tree-gap: 0.5rem;
}

.move-tree {
  display: flex;
  flex-direction: row;
  flex-wrap: wrap;

  gap: var(--move-tree-gap);
}

.move-tree-item {
  display: flex;
  flex-direction: row;
  align-items: center;
  flex-wrap: nowrap;

  gap: var(--move-tree-gap);
}

.move-tree-item-separator {
  background-color: var(--p-primary-200);

  width: 0.5rem;
  height: 0.15rem;
}

.turn-separator {
  /* background-color: var(--p-primary-color);
  height: 1rem;
  width: 0.15rem;

  margin: 0 0.5rem; */
  visibility: hidden;
}
</style>
