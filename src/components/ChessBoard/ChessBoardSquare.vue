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
      v-if="shouldDisplayCoordinate"
      class="absolute bottom-0 left-1 text-xs text-white/40 font-bold z-0"
    >
      {{ coordinateText }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import type { BoardTheme } from "../../shared/types";
import { boardToAlgebraic } from "./utils";

const props = defineProps<{
  row: number; // Row (0-7, 0 = top)
  col: number; // Column (0-7, 0 = left)
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

/**
 * Determines whether this square should display a coordinate label
 * Coordinates are shown on the left and bottom edges of the board
 */
const shouldDisplayCoordinate = computed(() => {
  if (!props.boardTheme.displayCoordinates) return false;
  // Show coordinates on the bottom row and left column (or top/right if flipped)
  return props.isBoardFlipped
    ? props.row === 0 || props.col === 7
    : props.row === 7 || props.col === 0;
});

/**
 * Generates the coordinate text (like "a1", "e4") for this square
 * Uses algebraic notation for the display
 */
const coordinateText = computed(() => {
  if (!props.boardTheme.displayCoordinates) return "";

  const algebraic = boardToAlgebraic(props.row, props.col);
  const file = algebraic[0];
  const rank = algebraic[1];

  const isBottomRow = props.isBoardFlipped ? props.row === 0 : props.row === 7;
  const isLeftCol = props.isBoardFlipped ? props.col === 7 : props.col === 0;

  if (isBottomRow && isLeftCol) return algebraic;
  if (isBottomRow) return file;
  if (isLeftCol) return rank;
  return "";
});

const squareStyle = computed(() => ({
  backgroundColor:
    (props.row + props.col) % 2 === 0
      ? props.boardTheme.lightSquare
      : props.boardTheme.darkSquare,
  width: `${props.squareSize}px`,
  height: `${props.squareSize}px`,
}));
</script>
