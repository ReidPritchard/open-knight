<template>
  <div class="flex flex-col h-full bg-base-200 select-none">
    <!-- Header/Toolbar (filter, sort, etc.) -->
    <div class="flex gap-4 p-1 justify-center">
      <!-- Filter dropdown -->
      <div class="dropdown">
        <label tabindex="0" class="btn m-1">Filter</label>
        <ul
          tabindex="0"
          class="dropdown-content z-10 menu p-2 shadow-sm bg-base-300 rounded-box w-52"
        >
          <li><a @click="setFilter('all')">All Games</a></li>
          <li><a @click="setFilter('my_games')">My Games</a></li>
          <li><a @click="setFilter('wins')">Wins</a></li>
          <li><a @click="setFilter('losses')">Losses</a></li>
          <li><a @click="setFilter('draws')">Draws</a></li>
        </ul>
      </div>

      <!-- Sort controls -->
      <select class="select w-full max-w-xs bg-base-300" v-model="currentSort">
        <option value="date_desc">Date (Newest)</option>
        <option value="date_asc">Date (Oldest)</option>
        <option value="white">White Player</option>
        <option value="black">Black Player</option>
        <option value="result">Result</option>
      </select>
    </div>

    <div ref="tableContainer" class="table-container overflow-x-auto grow">
      <table class="table table-pin-rows table-pin-cols">
        <thead>
          <tr>
            <th>White</th>
            <th>Black</th>
            <th>Result</th>
            <th>Opening</th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(game, index) in gameList.slice(
              pageOffset,
              pageOffset + pageSize
            )"
            :key="game.id"
            class="hover:bg-primary/75 hover:text-primary-content cursor-pointer transition-colors duration-50"
            :class="{
              'bg-base-300/90': index % 2 === 0,
              'bg-base-300/10': index % 2 === 1,
            }"
            @click="openGame(game.id)"
          >
            <td>{{ game.white_player.name }}</td>
            <td>{{ game.black_player.name }}</td>
            <td>{{ game.result }}</td>
            <td>{{ game.opening }}</td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Footer (pagination, etc.) -->
    <div class="flex gap-4 p-1 bg-base-200 justify-center">
      <div class="join">
        <button
          class="join-item btn"
          :disabled="page === 1"
          @click="pageDecrement"
        >
          «
        </button>
        <button class="join-item btn">Page {{ page }}/{{ pageCount }}</button>
        <button
          class="join-item btn"
          :disabled="page === pageCount"
          @click="pageIncrement"
        >
          »
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useGlobalStore } from "../../stores";

const globalStore = useGlobalStore();
const uiStore = globalStore.uiStore;
const gamesStore = globalStore.gamesStore;

// Add new reactive references for filter and sort
const currentSort = ref("date_desc");
const currentFilter = ref("all");

// Filter function
const setFilter = (filterType: string) => {
  currentFilter.value = filterType;
};

// Computed property for filtered and sorted games
const gameList = computed(() => {
  let games = [...globalStore.explorerGames];

  // Apply filter
  if (currentFilter.value !== "all") {
    games = games.filter((game) => {
      switch (currentFilter.value) {
        case "my_games":
          return (
            game.white_player.name === "You" || game.black_player.name === "You"
          );
        case "wins":
          return (
            (game.white_player.name === "You" && game.result === "1-0") ||
            (game.black_player.name === "You" && game.result === "0-1")
          );
        case "losses":
          return (
            (game.white_player.name === "You" && game.result === "0-1") ||
            (game.black_player.name === "You" && game.result === "1-0")
          );
        case "draws":
          return game.result === "1/2-1/2";
        default:
          return true;
      }
    });
  }

  // Apply sort
  games.sort((a, b) => {
    switch (currentSort.value) {
      case "date_desc":
      case "date_asc": {
        // Assuming dates are stored as ISO strings, convert to timestamps for comparison
        const dateA = new Date(a.date || "").getTime();
        const dateB = new Date(b.date || "").getTime();
        return currentSort.value === "date_desc"
          ? dateB - dateA
          : dateA - dateB;
      }
      case "white":
        return a.white_player.name.localeCompare(b.white_player.name);
      case "black":
        return a.black_player.name.localeCompare(b.black_player.name);
      case "result":
        return a.result.localeCompare(b.result);
      default:
        return 0;
    }
  });

  return games;
});

// const layout = computed(() => uiStore.gameLibraryView);
const sortBy = computed(() => uiStore.gameLibraryViewSortBy);
const sortOrder = computed(() => uiStore.gameLibraryViewSortOrder);
const filterTags = computed(() => uiStore.gameLibraryViewFilterTags);

// The page will be local state since it should be reset when the view is closed
const page = ref(1);
const tableContainer = ref<HTMLDivElement | null>(null);
// TODO: Improve this calculation
// maybe we could render the rows one at a time
// until one is outside of the viewport then split the page there
const pageSize = computed(() => {
  // biome-ignore lint/complexity/noExtraBooleanCast: <explanation>
  if (!!tableContainer?.value) {
    // The largest row I've seen is 105px (likely larger rows are possible)
    return Math.floor(tableContainer.value.clientHeight / 95);
  }
  return 10;
});
const pageCount = computed(() =>
  Math.max(1, Math.ceil(gameList.value.length / pageSize.value))
);

const pageDecrement = () => {
  page.value = Math.max(1, page.value - 1);
};

const pageIncrement = () => {
  page.value = Math.min(pageCount.value, page.value + 1);
};

const pageOffset = computed(() => (page.value - 1) * pageSize.value);

const openGame = (gameId: number) => {
  // TODO: Check if the game should be opened in a new board or replace an existing one
  console.log("openGame", gameId);
  gamesStore.openGame(gameId, 0);
};
</script>
