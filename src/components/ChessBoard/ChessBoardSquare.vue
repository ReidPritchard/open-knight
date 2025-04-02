<template>
  <div
    class="flex items-center justify-center relative"
    :class="{
      'bg-slate-400': (row + col) % 2 === 0,
      'bg-slate-600': (row + col) % 2 === 1,
      'cursor-grab': canMove,
    }"
    :style="`width: ${squareSize}px; height: ${squareSize}px;`"
    @drop="$emit('drop')"
    @dragover.prevent
    @click="$emit('click', $event)"
  >
    <!-- Chess Piece -->
    <img
      v-if="piece"
      :src="pieceImage"
      class="select-none"
      :class="{ 'ring-2 ring-primary': isSelected }"
      :style="`width: ${squareSize * 0.75}px; height: ${squareSize * 0.75}px;`"
      :draggable="canMove"
      @dragstart="$emit('dragStart')"
      alt="Chess piece"
    />

    <!-- Valid Move Indicator -->
    <div
      v-if="isValidMove"
      class="absolute bg-success/80 rounded-full"
      :style="`width: ${squareSize * 0.2}px; height: ${
        squareSize * 0.2
      }px; margin: auto;`"
    ></div>

    <!-- Current Move Highlight -->
    <div v-if="isHighlighted" class="absolute inset-0 bg-success"></div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  row: number;
  col: number;
  squareSize: number;
  piece?: string;
  pieceImage?: string;
  canMove: boolean;
  isSelected: boolean;
  isValidMove: boolean;
  isHighlighted: boolean;
}>();

defineEmits<{
  (e: "drop"): void;
  (e: "click", event: MouseEvent): void;
  (e: "dragStart"): void;
}>();
</script>
