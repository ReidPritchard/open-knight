<template>
  <div class="move-tree">
    <!-- Move tree content -->
    <p>Move Tree</p>
    <div v-for="move in moves" :key="move.id">
      <p>{{ move.move_san }}</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useGlobalState } from "../shared/store";

const { selectedGame } = useGlobalState();

const moves = computed(() => selectedGame.value?.moves);
</script>

<style scoped>
.move-tree {
  background-color: #f5f5f5;
  padding: 10px;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;

  overflow-y: scroll;
}

.move-tree p {
  margin: 0;
}

.move-tree div {
  margin-left: 1rem;
}

.move-tree div:nth-child(even) {
  background-color: #e0e0e0;
}

.move-tree div:nth-child(odd) {
  background-color: #f5f5f5;
}
</style>
