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
import { IGame } from "../shared/types";
import { useGlobalState } from "../shared/store";

const { games, selectedGame } = useGlobalState();

function gameSelectionChanged(
  event: DataTableRowUnselectEvent | DataTableRowSelectEvent,
) {
  const rowSelection = event.data as IGame;
  useGlobalState().setSelectedGame(rowSelection);
}

let pgnInput = ref("");

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
          sortField="headers.date"
          sortMode="multiple"
        >
          <Column field="headers.date" header="Date" sortable></Column>
          <Column field="headers.event" header="Event" sortable></Column>
          <Column field="headers.site" header="Site" sortable></Column>
          <Column field="headers.round" header="Round" sortable></Column>
          <Column field="headers.white" header="White" sortable></Column>
          <Column field="headers.black" header="Black" sortable></Column>
          <Column field="headers.result" header="Result" sortable></Column>
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
