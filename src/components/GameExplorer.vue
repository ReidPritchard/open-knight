<template>
  <div class="w-full h-full bg-white dark:bg-gray-800 rounded-lg shadow-sm p-4">
    <!-- No Games State -->
    <div v-if="!gameStore.hasGames" class="space-y-4">
      <p class="text-gray-600 dark:text-gray-400">
        No games loaded. Please load a PGN file.
      </p>

      <div class="flex gap-4">
        <div class="relative flex-1">
          <input
            id="pgn-input"
            v-model="pgnInput"
            type="text"
            class="w-full px-3 py-2 border rounded-md text-gray-900 dark:text-gray-100 bg-white dark:bg-gray-700 border-gray-300 dark:border-gray-600 focus:ring-2 focus:ring-blue-500 focus:border-blue-500"
            :class="{ 'border-blue-500': pgnInput }"
          />
          <label
            for="pgn-input"
            class="absolute left-3 -top-2.5 px-1 text-xs text-gray-600 dark:text-gray-400 bg-white dark:bg-gray-800"
            >PGN</label
          >
        </div>
        <button
          @click="parsePgn"
          :disabled="!pgnInput.trim() || isLoading"
          class="px-4 py-2 text-sm font-medium text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          <template v-if="isLoading">
            <svg
              class="w-5 h-5 animate-spin"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 24 24"
            >
              <circle
                class="opacity-25"
                cx="12"
                cy="12"
                r="10"
                stroke="currentColor"
                stroke-width="4"
              ></circle>
              <path
                class="opacity-75"
                fill="currentColor"
                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
              ></path>
            </svg>
          </template>
          <span v-else>Parse</span>
        </button>
      </div>
    </div>

    <!-- Games Table -->
    <div v-else class="relative overflow-x-auto">
      <div
        v-if="isLoading"
        class="absolute inset-0 bg-white/50 dark:bg-gray-800/50 flex items-center justify-center z-10"
      >
        <svg
          class="w-8 h-8 animate-spin text-blue-600"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
        >
          <circle
            class="opacity-25"
            cx="12"
            cy="12"
            r="10"
            stroke="currentColor"
            stroke-width="4"
          ></circle>
          <path
            class="opacity-75"
            fill="currentColor"
            d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
          ></path>
        </svg>
      </div>

      <table class="w-full text-sm text-left text-gray-600 dark:text-gray-400">
        <thead class="text-xs uppercase bg-gray-100 dark:bg-gray-700">
          <tr>
            <th
              v-for="header in uiStore.visibleGameHeaders"
              :key="header"
              scope="col"
              class="px-4 py-3 font-medium cursor-pointer select-none"
              @click="toggleSort(header)"
            >
              <div class="flex items-center gap-2">
                {{ header }}
                <svg
                  v-if="sortConfig.key === header"
                  class="w-4 h-4"
                  :class="{ 'rotate-180': sortConfig.direction === 'desc' }"
                  xmlns="http://www.w3.org/2000/svg"
                  viewBox="0 0 20 20"
                  fill="currentColor"
                >
                  <path
                    fill-rule="evenodd"
                    d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
                    clip-rule="evenodd"
                  />
                </svg>
              </div>
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="game in sortedGames"
            :key="game.game.id ?? game.game.pgn"
            class="border-b dark:border-gray-700 hover:bg-gray-50 dark:hover:bg-gray-600 cursor-pointer"
            :class="{
              'bg-gray-50 dark:bg-gray-900': isEvenRow(game),
              'bg-blue-500 dark:bg-blue-600':
                gameStore.selectedGame?.game &&
                gameStore.selectedGame?.game.id === game.game.id,
            }"
            @click="selectGame(game)"
          >
            <td
              v-for="header in uiStore.visibleGameHeaders"
              :key="header"
              class="px-4 py-3 whitespace-nowrap"
            >
              {{ getHeaderValue(game, header) }}
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import type { ExplorerGame } from "../shared/bindings/ExplorerGame";
import { useGameStore } from "../stores/game";
import { useUIStore } from "../stores/ui";

const gameStore = useGameStore();
const uiStore = useUIStore();

const pgnInput = ref("");
const isLoading = ref(false);

const getHeaderValue = (game: ExplorerGame, header: string): string => {
  return (
    game.headers.find((searchHeader) => searchHeader.header_key === header)
      ?.header_value ?? ""
  );
};

// Sorting
interface SortConfig {
  key: string;
  direction: "asc" | "desc";
}

const sortConfig = ref<SortConfig>({
  key: "Date",
  direction: "desc",
});

const toggleSort = (header: string) => {
  if (sortConfig.value.key === header) {
    sortConfig.value.direction =
      sortConfig.value.direction === "asc" ? "desc" : "asc";
  } else {
    sortConfig.value = {
      key: header,
      direction: "asc",
    };
  }
};

const sortedGames = computed(() => {
  return [...gameStore.games].sort((a, b) => {
    const aValue = getHeaderValue(a, sortConfig.value.key);
    const bValue = getHeaderValue(b, sortConfig.value.key);

    const comparison = aValue.localeCompare(bValue);
    return sortConfig.value.direction === "asc" ? comparison : -comparison;
  });
});

const isEvenRow = (game: ExplorerGame) => {
  return sortedGames.value.indexOf(game) % 2 === 0;
};

// Game Selection
const selectGame = async (game: ExplorerGame) => {
  isLoading.value = true;
  try {
    console.log("GameExplorer: Starting game selection for game:", game);
    await gameStore.setSelectedGame(game);
    console.log("GameExplorer: Game selection completed");
  } catch (error) {
    console.error("GameExplorer: Error selecting game:", error);
    // TODO: Add user-facing error handling here
  } finally {
    isLoading.value = false;
  }
};

// PGN Parsing
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

<style scoped>
.game-explorer {
  user-select: none;
  max-width: 100%;
  max-height: 100%;
}
</style>
