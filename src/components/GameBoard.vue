<script setup lang="ts">
import { ref } from "vue";
import { useGlobalStore } from "../stores";
import AspectRatio from "./AspectRatio.vue";
import ChessBoard from "./ChessBoard/ChessBoard.vue";
import {
  CoordinatesStyle,
  type CoordinatesStyleType,
  type Orientation,
} from "./ChessBoard/types";

const { game } = useGlobalStore();

const showCoordinates = ref<CoordinatesStyleType>(CoordinatesStyle.none);

const orientation = ref<Orientation>("white");

const previousMove = () => {
  const newLocation = (game.selectedGameLocation ?? 0) - 1;
  game.setSelectedGameLocation(Math.max(newLocation, 0));
};

const nextMove = () => {
  const newLocation = (game.selectedGameLocation ?? 0) + 1;
  game.setSelectedGameLocation(
    Math.min(newLocation, (game.selectedGame?.moves?.length ?? 0) - 1)
  );
};
</script>

<template>
  <div class="w-full rounded-lg bg-white dark:bg-gray-800 shadow-md p-4">
    <div
      v-if="game.selectedGame"
      class="flex flex-col gap-4 max-w-[70vw] max-h-[70vh]"
    >
      <AspectRatio ratio="1 / 1">
        <ChessBoard
          initial-position="start"
          :current-position="
            game.selectedGame?.moves?.[game.selectedGameLocation ?? 0]
              ?.parent_position.fen ?? 'start'
          "
          :orientation="orientation"
          :show-coordinates="showCoordinates"
          :draggable="true"
        />
      </AspectRatio>

      <nav
        class="flex items-center justify-between p-4 bg-gray-50 dark:bg-gray-700 rounded-md"
      >
        <div class="flex-1">
          <!-- Left section (empty for now, previously debugging tools) -->
        </div>

        <div class="flex-1 flex items-center justify-center gap-2">
          <button
            @click="previousMove"
            class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path
                fill-rule="evenodd"
                d="M12.707 5.293a1 1 0 010 1.414L9.414 10l3.293 3.293a1 1 0 01-1.414 1.414l-4-4a1 1 0 010-1.414l4-4a1 1 0 011.414 0z"
                clip-rule="evenodd"
              />
            </svg>
          </button>

          <span>{{ game.selectedGameLocation }}</span>
          <span>/</span>
          <span>{{ game.selectedGame?.moves?.length }}</span>

          <button
            @click="nextMove"
            class="p-2 rounded-full hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              class="h-5 w-5"
              viewBox="0 0 20 20"
              fill="currentColor"
            >
              <path
                fill-rule="evenodd"
                d="M7.293 14.707a1 1 0 010-1.414L10.586 10 7.293 6.707a1 1 0 011.414-1.414l4 4a1 1 0 010 1.414l-4 4a1 1 0 01-1.414 0z"
                clip-rule="evenodd"
              />
            </svg>
          </button>

          <span class="ml-2 text-sm">
            {{
              game.selectedGame?.moves?.[game.selectedGameLocation ?? 0]
                ?.game_move.move_san
            }}
          </span>
        </div>

        <div class="flex-1">
          <!-- Right section (empty for now) -->
        </div>
      </nav>
    </div>
    <div v-else class="text-gray-500 dark:text-gray-400">No game selected</div>
  </div>
</template>

<style scoped>
#game-board-container {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  color: var(--p-card-color);
  background-color: var(--p-card-background);
  max-width: 70vw;
  max-height: 70vh;
}

.game-board {
  /* width: 100% removed */
}

.game-board-move-number {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}
</style>
