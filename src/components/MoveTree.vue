<template>
  <div class="move-tree">
    <OrganizationChart :value="moves">
      <template #default="slotProps">
        <span>{{ slotProps.node.label }}</span>
      </template>
    </OrganizationChart>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useGlobalState } from "../shared/store";
import OrganizationChart, {
  OrganizationChartNode,
} from "primevue/organizationchart";

const { selectedGame } = useGlobalState();

const moves = computed(() => {
  const gameMoves = selectedGame.value?.moves ?? [];
  const moveTree: OrganizationChartNode = {
    key: "root",
    label: "Root",
    children: [],
  };

  let currentNode = moveTree;

  for (const move of gameMoves) {
    currentNode.children = currentNode.children ?? [];
    currentNode.children.push({
      key: move.id,
      label: move.move_san,
      children: [],
    });
    currentNode = currentNode.children[currentNode.children.length - 1];
  }

  return moveTree;
});
</script>

<style scoped>
.move-tree {
  background-color: var(--p-surface-color);
  padding: 0.5rem;

  max-height: 500px;

  overflow-y: scroll;
}
</style>
