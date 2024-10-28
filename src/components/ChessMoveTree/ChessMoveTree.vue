<template>
  <div>
    <ChessMove
      v-for="move in moveTree"
      :key="move.move_number"
      :move="move"
      @move-click="handleMoveClick"
    />
  </div>
</template>

<script setup lang="ts">
import { IMove } from "../../shared/types";
import ChessMove from "./ChessMove.vue";

const props = defineProps<{
  moves: IMove[];
}>();

const buildMoveTree = (moves: IMove[]) => {
  // Move tree is a nested object where each move is a node and the children are the moves that follow it
  // The root node is the first move in the list
  // The children of a node are the moves that follow it in the list or are in a variation that starts with it
  // The tree is built by iterating through the list of moves and adding each move to the tree
  // The tree is then returned
  const moveTree: Array<IMove & { children: IMove[] }> = [];
  const visitedMoves: Record<string, boolean> = {};

  moves.forEach((move) => {
    if (!visitedMoves[move.move_number]) {
      visitedMoves[move.move_number] = true;
      moveTree.push({ ...move, children: [] });
    } else if (move.variation_id) {
      const variationMove = moves.find(
        (m) => m.variation_id === move.variation_id
      );
      if (variationMove) {
        moveTree
          .find((m) => m.move_number === variationMove.move_number)
          ?.children.push(variationMove);
      }
    }
  });

  return moveTree;
};

const moveTree = buildMoveTree(props.moves);

const handleMoveClick = (move: IMove) => {
  console.log(move);
};
</script>
