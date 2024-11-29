<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from "vue";
import { useGlobalStore } from "../stores";
import ChessBoard from "./ChessBoard/ChessBoard.vue";
import {
  CoordinatesStyle,
  type CoordinatesStyleType,
  type Orientation,
} from "./ChessBoard/types";

const { game } = useGlobalStore();

// Add watchers for game state changes
watch(
  () => game.selectedGame,
  (newGame) => {
    console.log("Game changed:", newGame);
    if (newGame) {
      game.setSelectedGameLocation(0); // Reset to first move when game changes
    }
  },
  { deep: true }
);

watch(
  () => game.selectedGameLocation,
  (newLocation) => {
    console.log("Move location changed:", newLocation);
    console.log("Current position:", game.currentPosition);
  }
);

const showCoordinates = ref<CoordinatesStyleType>(CoordinatesStyle.none);

const orientation = ref<Orientation>("white");

const previousMove = () => {
  game.goToPreviousMove();
};

const nextMove = () => {
  game.goToNextMove();
};

// Add keyboard navigation
const handleKeyDown = (event: KeyboardEvent) => {
  if (event.key === "ArrowLeft") {
    previousMove();
  } else if (event.key === "ArrowRight") {
    nextMove();
  }
};

onMounted(() => {
  window.addEventListener("keydown", handleKeyDown);
});

onUnmounted(() => {
  window.removeEventListener("keydown", handleKeyDown);
});
</script>

<template>
  <div class="w-full h-full rounded-lg bg-white dark:bg-gray-800 shadow-md p-4">
    <div v-if="game.selectedGame" class="h-full flex flex-col gap-4">
      <!-- Chess board container with proper spacing -->
      <div class="flex-1 min-h-0 flex items-center justify-center">
        <div
          class="relative"
          style="width: min(100%, calc(100vh - 16rem)); aspect-ratio: 1"
        >
          <ChessBoard
            class="w-full h-full"
            initial-position="start"
            :current-position="game.currentPosition"
            :orientation="orientation"
            :show-coordinates="showCoordinates"
            :draggable="true"
          />
        </div>
      </div>

      <!-- Navigation bar -->
      <div class="flex-shrink-0">
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
            <span>{{ game.totalMoves }}</span>

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
              {{ game.currentMove }}
            </span>
          </div>

          <div class="flex-1">
            <div class="flex items-center gap-2">
              <button
                @click="
                  orientation = orientation === 'white' ? 'black' : 'white'
                "
                class="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                title="Flip board"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-5 w-5"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fill-rule="evenodd"
                    d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z"
                    clip-rule="evenodd"
                  />
                </svg>
              </button>
              <button
                @click="
                  showCoordinates =
                    showCoordinates === 'none' ? 'outside' : 'none'
                "
                class="p-2 rounded-md hover:bg-gray-200 dark:hover:bg-gray-600 transition-colors"
                title="Toggle coordinates"
              >
                <svg
                  xmlns="http://www.w3.org/2000/svg"
                  class="h-5 w-5"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    d="M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z"
                  />
                </svg>
              </button>
            </div>
          </div>
        </nav>
      </div>
    </div>
    <div v-else class="text-gray-500 dark:text-gray-400">No game selected</div>
  </div>
</template>

<style scoped>
/* No styles needed - using Tailwind */
</style>
