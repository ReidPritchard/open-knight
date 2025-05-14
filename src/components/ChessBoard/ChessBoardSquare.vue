<template>
  <div
    class="flex items-center justify-center relative"
    :class="{
      'cursor-grab': canMove,
    }"
    :style="squareStyle"
    @drop="$emit('drop')"
    @dragover.prevent
    @click="$emit('click', $event)"
  >
    <!-- Chess Piece -->
    <img
      v-if="piece"
      :src="pieceImage"
      class="select-none z-10"
      :class="{ 'ring-2 ring-primary': isSelected }"
      :style="`width: ${squareSize * 0.75}px; height: ${squareSize * 0.75}px;`"
      :draggable="canMove"
      @dragstart="$emit('dragStart')"
      alt="Chess piece"
    />

    <!-- Valid Move Indicator -->
    <div
      v-if="isValidMove"
      class="absolute bg-success/80 rounded-full z-20"
      :style="`width: ${squareSize * 0.2}px; height: ${
        squareSize * 0.2
      }px; margin: auto;`"
    ></div>

    <!-- Current Move Highlight -->
    <div v-if="isHighlighted" class="absolute inset-0 bg-info/50 z-0"></div>

    <!-- Coordinates -->
    <div
      v-if="displayCoordinates"
      class="absolute bottom-0 left-1 text-xs text-white/40 font-bold z-0"
    >
      {{ coordinateText }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { BoardTheme } from "../../shared/types";
import { coordsToAlgebraic } from "./utils";

const props = defineProps<{
  row: number;
  col: number;
  squareSize: number;
  piece?: string;
  pieceImage?: string;
  canMove: boolean;
  isSelected: boolean;
  isValidMove: boolean;
  isHighlighted: boolean;
  isBoardFlipped: boolean;
  boardTheme: BoardTheme;
}>();

defineEmits<{
  (e: "drop"): void;
  (e: "click", event: MouseEvent): void;
  (e: "dragStart"): void;
}>();

const displayCoordinates = computed(() => {
  if (!props.boardTheme.displayCoordinates) return false;

  // Only display coordinates on the left and bottom edges
  // account for rotation for determining what row/col index to display
  if (props.isBoardFlipped) {
    return props.row === 0 || props.col === 7;
  }
  return props.row === 7 || props.col === 0;
});

const coordinateText = computed(() => {
  if (!props.boardTheme.displayCoordinates) return "";

  const boardRow = 7 - props.row;
  const boardCol = 7 - props.col;

  return coordsToAlgebraic(boardCol, boardRow);
});

const squareStyle = computed(() => {
  return {
    backgroundColor:
      props.row % 2 === props.col % 2
        ? props.boardTheme.lightSquare
        : props.boardTheme.darkSquare,
    width: `${props.squareSize}px`,
    height: `${props.squareSize}px`,
  };
});
</script>
