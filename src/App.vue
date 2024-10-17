<script setup lang="ts">
import GameExplorer from "./components/GameExplorer.vue";
import GameBoard from "./components/GameBoard.vue";
import { ref, watch } from "vue";
import { Chess } from "chess.js";

export type Game = {
  id: string;
  headers: Record<string, string>;
  game: Chess;
  pgn: string;
  errors: string[];
}

export type ExplorerState = {
  games: Game[];
}

export type GameBoardGame = Omit<Game, 'game'>;
const selectedGame = ref<GameBoardGame | null>(null);

async function updateSelectedGame(newGame: GameBoardGame) {
  selectedGame.value = newGame;
  console.log(selectedGame.value);
}

</script>

<template>
  <div class="container">
    <h1>Welcome to Open Knight 🐴</h1>

    <GameExplorer @update:selectedGame="updateSelectedGame" />
    <GameBoard v-if="selectedGame" />
  </div>
</template>

<style scoped>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 14px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 5vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
