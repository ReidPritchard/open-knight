<script setup lang="ts">
import Button from "primevue/button";
import Card from "primevue/card";
import Column from "primevue/column";
import type {
  DataTableRowSelectEvent,
  DataTableRowUnselectEvent,
} from "primevue/datatable";
import DataTable from "primevue/datatable";
import FloatLabel from "primevue/floatlabel";
import InputText from "primevue/inputtext";
import { ref } from "vue";
import type { ExplorerGame } from "../shared/bindings/ExplorerGame";
import { useGameStore } from "../stores/game";
import { useUIStore } from "../stores/ui";

const gameStore = useGameStore();
const uiStore = useUIStore();

const pgnInput = ref("");
const isLoading = ref(false);

async function gameSelectionChanged(
  event: DataTableRowSelectEvent | DataTableRowUnselectEvent
) {
  const rowSelection = event.data as ExplorerGame;
  isLoading.value = true;
  try {
    await gameStore.setSelectedGame(rowSelection);
  } finally {
    isLoading.value = false;
  }
}

const parsePgn = async () => {
  isLoading.value = true;
  try {
    await gameStore.parsePgnText(pgnInput.value);
    pgnInput.value = ""; // Clear input after successful parse
  } finally {
    isLoading.value = false;
  }
};
</script>

<template>
  <Card class="game-explorer">
    <template #content>
      <div v-if="!gameStore.hasGames">
        <p>No games loaded. Please load a PGN file.</p>

        <div style="display: flex; flex-direction: row; gap: 1rem">
          <FloatLabel variant="on">
            <InputText id="pgn-input" v-model="pgnInput" />
            <label for="pgn-input">PGN</label>
          </FloatLabel>
          <Button
            type="submit"
            @click="parsePgn"
            :loading="isLoading"
            :disabled="!pgnInput.trim()"
          >
            Parse
          </Button>
        </div>
      </div>

      <div v-else style="display: flex; flex-direction: column; gap: 1rem">
        <DataTable
          :value="gameStore.games"
          v-model:selection="gameStore.selectedGame"
          tableStyle="min-width: 50rem"
          stripedRows
          showGridlines
          selectionMode="single"
          dataKey="id"
          @rowSelect="gameSelectionChanged"
          @rowUnselect="gameSelectionChanged"
          sortMode="multiple"
          :loading="isLoading"
        >
          <Column
            v-for="col of uiStore.visibleGameHeaders"
            :key="col"
            :field="col"
            :header="col"
          >
            <template #body="slotProps">
              {{ slotProps.data.headers[col] }}
            </template>
          </Column>
        </DataTable>
      </div>
    </template>
  </Card>
</template>

<style scoped>
.game-explorer {
  user-select: none;
  max-width: 100%;
  max-height: 100%;
}
</style>
