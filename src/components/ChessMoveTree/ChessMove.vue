<template>
  <div @click="handleClick">
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
