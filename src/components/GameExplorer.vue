<script setup lang="ts">
import Button from "primevue/button";
import Card from "primevue/card";
import Column from "primevue/column";
import DataTable, {
  DataTableRowSelectEvent,
  DataTableRowUnselectEvent,
} from "primevue/datatable";
import FloatLabel from "primevue/floatlabel";
import InputText from "primevue/inputtext";
import { ref } from "vue";
import { useGlobalState } from "../shared/store";
import { IGame } from "../shared/types";

const { games, selectedGame, UIState } = useGlobalState();

let pgnInput = ref("");

function gameSelectionChanged(
  event: DataTableRowUnselectEvent | DataTableRowSelectEvent,
) {
  const rowSelection = event.data as IGame;
  useGlobalState().setSelectedGame(rowSelection);
}

const parsePgn = () => {
  useGlobalState().parsePgnText(pgnInput.value);
};
</script>

<template>
  <Card style="width: 100%" class="game-explorer">
    <template #content>
      <div v-if="games && games.length === 0">
        <p>No games loaded. Please load a PGN file.</p>

        <div style="display: flex; flex-direction: row; gap: 1rem">
          <FloatLabel variant="on">
            <InputText id="pgn-input" v-model="pgnInput" />
            <label for="pgn-input">PGN</label>
          </FloatLabel>
          <Button type="submit" @click="parsePgn">Parse</Button>
        </div>
      </div>

      <div v-else style="display: flex; flex-direction: column; gap: 1rem">
        <DataTable
          :value="games"
          v-model:selection="selectedGame"
          tableStyle="min-width: 50rem"
          stripedRows
          showGridlines
          selectionMode="single"
          dataKey="id"
          @rowSelect="gameSelectionChanged"
          @rowUnselect="gameSelectionChanged"
          sortMode="multiple"
        >
          <Column
            v-for="col of UIState.visibleGameHeaders"
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
}
</style>
