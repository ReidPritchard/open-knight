<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { Chess } from "chess.js";

type ExplorerState = {
  games: Game[];
}

type Game = {
  headers: Record<string, string>;
  game: Chess;
  pgn: string;
  errors: string[];
}

const games = ref<Game[]>([]);
const pgn = ref("");

async function updateGames() {
  const state: string = await invoke("get_explorer_state");
  const parsedState: ExplorerState = JSON.parse(state);
  games.value = parsedState.games.map((game) => {
    const loaded_game = new Chess();
    loaded_game.loadPgn(game.pgn);
    return {
      headers: game.headers,
      game: loaded_game,
      pgn: game.pgn,
      errors: game.errors,
    }
  });
}

async function parsePgn() {
  await invoke("parse_pgn", { pgn: pgn.value });
  await updateGames();
}

onMounted(async () => {
  await updateGames();
});


</script>

<template>
  <!-- If no games are loaded, prompt user to paste a PGN file -->
  <div v-if="games.length === 0">
    <p>No games loaded. Please load a PGN file.</p>

    <input id="pgn-input" v-model="pgn" placeholder="Enter PGN..." />
    <button type="submit" @click="parsePgn">Parse</button>
  </div>

  <div v-else>
    <table>
      <tr>
        <th>Headers</th>
        <th>PGN</th>
        <th>Errors</th>
      </tr>
      <tr v-for="game in games">
        <td>{{ game.headers }}</td>
        <td>{{ game.pgn }}</td>
        <td>{{ game.errors }}</td>
      </tr>
    </table>
  </div>


</template>
