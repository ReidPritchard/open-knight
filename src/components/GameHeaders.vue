<template>
  <div class="game-headers">
    <DataTable :value="headers" size="small" :rows="5" paginator>
      <Column field="key" header="Key" />
      <Column field="value" header="Value" />
    </DataTable>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useGlobalStore } from "../stores";

const { game } = useGlobalStore();

// An array of objects with two fields: key and value
const headers = computed(() => {
  const headers = game.selectedGame?.game_data ?? {};
  return Object.entries(headers).map(([key, value]) => ({
    key,
    value,
  }));
});
</script>

<style scoped>
.game-headers {
  background-color: var(--p-surface-color);
  padding: 0.5rem;

  max-width: 400px;

  --p-datatable-header-padding: 0;
  --p-datatable-header-cell-padding: 0.25rem;
  --p-datatable-body-cell-padding: 0.25rem;

  /* --p-datatable-column-title-font-weight: 200; */
}
</style>
