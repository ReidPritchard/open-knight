<!-- TODO: add UI option to use icons instead of text for pieces -->

<template>
  <div class="move" @click="handleClick">
    <!-- TODO: add syntax highlighting for the move san -->
    <div>{{ props.move.move_san }}</div>
  </div>
</template>

<script setup lang="ts">
import { IMove } from "../../shared/types";

const props = defineProps<{
  move: IMove;
}>();

const emit = defineEmits<{
  (e: "move-click", move: IMove): void;
}>();

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

.move:hover {
  background-color: var(--p-primary-200);
}
</style>
