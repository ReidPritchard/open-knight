<!-- TODO: add UI option to use icons instead of text for pieces -->

<template>
  <div
    class="move"
    :class="{ 'current-move': props.isCurrentMove }"
    @click="handleClick"
  >
    <!-- TODO: add syntax highlighting for the move san -->
    <div>{{ props.move.chess_move.move_san }}</div>
  </div>
</template>

<script setup lang="ts">
import type { IAPIMove } from "../../shared/types";

const props = defineProps<{
  move: IAPIMove;
  isCurrentMove: boolean;
}>();

const emit = defineEmits<(e: "move-click", move: IAPIMove) => void>();

const handleClick = () => {
  emit("move-click", props.move);
};
</script>

<style scoped>
.move {
  cursor: pointer;
  border-bottom: 2px solid var(--p-primary-200);
  padding: 0.15rem;
  border-radius: var(--p-border-radius-md);

  transition: background-color 0.2s ease-in-out;
}

.move.current-move {
  background-color: var(--p-primary-200);
}

.move:hover {
  background-color: var(--p-primary-200);
}
</style>
