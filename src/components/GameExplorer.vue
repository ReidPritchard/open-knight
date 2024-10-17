<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import Card from 'primevue/card';
import DataTable, { DataTableRowSelectEvent, DataTableRowUnselectEvent } from 'primevue/datatable';
import Column from 'primevue/column';
import { ExplorerState, Game, GameBoardGame } from "../App.vue";
import { apiExplorerStateToExplorerState, apiSelectedGameToGame } from "../shared/api-conversions";

const emit = defineEmits(['update:selectedGame']);

const selectedGame = defineModel<GameBoardGame | null>('selectedGame');

const games = ref<Game[]>([]);
const pgn = ref("");

async function updateGames() {
  const state: string = await invoke("get_explorer_state");
  const parsedState: ExplorerState = apiExplorerStateToExplorerState(state);
  games.value = parsedState.games;
}

async function getSelectedGame() {
  const game: string = await invoke("get_selected_game");
  const parsedGame: GameBoardGame = apiSelectedGameToGame(game);
  selectedGame.value = parsedGame;
}

async function parsePgn() {
  await invoke("parse_pgn", { pgn: pgn.value });
  await updateGames();
  await getSelectedGame();
}

async function gameSelectionChanged(event: DataTableRowUnselectEvent | DataTableRowSelectEvent) {
  const rowSelection = event.data;

  selectedGame.value = rowSelection;
  emit('update:selectedGame', rowSelection);

  if (rowSelection) {
    await invoke("set_selected_game", { gameId: rowSelection.id });
  }
}


onMounted(async () => {
  await updateGames();
  await getSelectedGame();
});

</script>

<template>
  <Card style="width: 100%;">

    <template #title>
      Explorer
    </template>

    <template #content>

      <!-- If no games are loaded, prompt user to paste a PGN file -->
      <div v-if="games.length === 0">
        <p>No games loaded. Please load a PGN file.</p>

        <input id="pgn-input" v-model="pgn" placeholder="Enter PGN..." />
        <button type="submit" @click="parsePgn">Parse</button>
      </div>

      <div v-else style="display: flex; flex-direction: column; gap: 1rem;">
        <DataTable v-model:selection="selectedGame" :value="games" tableStyle="min-width: 50rem" stripedRows
          showGridlines selectionMode="single" dataKey="id" @row-select="gameSelectionChanged"
          @row-unselect="gameSelectionChanged" sortField="headers.date" sortMode="multiple">
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
