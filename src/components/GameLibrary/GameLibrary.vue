<template>
  <div class="flex flex-col gap-4 bg-base-200">
    <!-- Header/Toolbar (search, filter, sort, etc.) -->
    <div class="flex flex-col sm:flex-row gap-4 justify-center pt-4">
      <!-- Filter dropdown -->
      <div class="dropdown">
        <label
          tabindex="0"
          class="btn btn-ghost btn-sm"
          aria-label="Filter games"
          aria-haspopup="true"
          aria-expanded="false"
        >
          Filter
          <span class="ml-1 text-xs opacity-70" v-if="currentFilter !== 'all'">
            ({{ getFilterLabel(currentFilter) }})
          </span>
        </label>
        <ul
          tabindex="0"
          class="dropdown-content z-10 menu p-2 shadow-sm bg-base-300 rounded-box w-52"
          role="menu"
        >
          <li role="none">
            <a @click="setFilter('all')" role="menuitem">All Games</a>
          </li>
          <li role="none">
            <a @click="setFilter('my_games')" role="menuitem">My Games</a>
          </li>
          <li role="none">
            <a @click="setFilter('wins')" role="menuitem">Wins</a>
          </li>
          <li role="none">
            <a @click="setFilter('losses')" role="menuitem">Losses</a>
          </li>
          <li role="none">
            <a @click="setFilter('draws')" role="menuitem">Draws</a>
          </li>
        </ul>
      </div>

      <!-- Search input -->
      <div class="form-control w-full max-w-xs">
        <input
          v-model="searchQuery"
          type="text"
          placeholder="Search..."
          class="input input-bordered w-full text-sm input-sm"
          aria-label="Search games by player name"
        />
      </div>
    </div>

    <!-- Loading state -->
    <div v-if="isLoading" class="flex-1 flex justify-center items-center p-8">
      <span class="loading loading-spinner loading-lg"></span>
    </div>

    <!-- Error state -->
    <div v-else-if="error" class="flex-1 flex justify-center items-center p-8">
      <div class="alert alert-error max-w-md">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          class="stroke-current shrink-0 h-6 w-6"
          fill="none"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
        <span>{{ error }}</span>
      </div>
    </div>

    <!-- Empty state -->
    <div
      v-else-if="gameList.length === 0"
      class="flex-1 flex justify-center items-center p-8"
    >
      <div class="text-center">
        <div class="text-6xl mb-4">♟️</div>
        <h3 class="text-lg font-semibold mb-2">No games found</h3>
        <p class="text-base-content/70">
          {{
            searchQuery || currentFilter !== "all"
              ? "Try adjusting your search or filter criteria"
              : "No games available yet"
          }}
        </p>
      </div>
    </div>

    <!-- Games table -->
    <div v-else class="flex-1 min-h-0">
      <table class="table table-sm table-pin-rows table-pin-cols" role="table">
        <thead>
          <tr role="row">
            <th
              scope="col"
              @click="setSort('date')"
              class="cursor-pointer hover:bg-info/10 flex gap-1 justify-between items-center"
              :class="{
                'bg-info/50':
                  currentSort === 'date_desc' || currentSort === 'date_asc',
              }"
            >
              Date
              <PhSortAscending v-if="currentSort === 'date_asc'" size="16" />
              <PhSortDescending v-if="currentSort === 'date_desc'" size="16" />
            </th>
            <th
              scope="col"
              @click="setSort('white')"
              class="cursor-pointer hover:bg-info/10"
              :class="{
                'bg-info/50': currentSort === 'white',
              }"
            >
              White
            </th>
            <th
              scope="col"
              @click="setSort('black')"
              class="cursor-pointer hover:bg-info/10"
              :class="{
                'bg-info/50': currentSort === 'black',
              }"
            >
              Black
            </th>
            <th
              scope="col"
              @click="setSort('result')"
              class="cursor-pointer hover:bg-info/10"
              :class="{
                'bg-info/50': currentSort === 'result',
              }"
            >
              Result
            </th>
          </tr>
        </thead>
        <tbody>
          <tr
            v-for="(game, index) in paginatedGames"
            :key="game.id"
            class="hover:bg-primary/75 hover:text-primary-content cursor-pointer transition-colors duration-150"
            :class="{
              'bg-base-300/90': index % 2 === 0,
              'bg-base-300/10': index % 2 === 1,
            }"
            @click="openGame(game.id)"
            @contextmenu.prevent="showContextMenu($event, game.id)"
            @keydown.enter="openGame(game.id)"
            @keydown.space.prevent="openGame(game.id)"
            tabindex="0"
            role="row"
            :aria-label="`Game between ${game.white_player.name} and ${game.black_player.name}, result ${game.result}`"
          >
            <td>{{ formatDate(game.date) }}</td>
            <td>{{ game.white_player.name }}</td>
            <td>{{ game.black_player.name }}</td>
            <td>
              <span
                class="badge badge-xs badge-neutral float-right break-keep"
                :class="{
                  'badge-success': game.result === '1-0',
                  'badge-error': game.result === '0-1',
                  'badge-warning': game.result === '1/2-1/2',
                }"
              >
                {{ game.result }}
              </span>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Footer (pagination, etc.) -->
    <div
      v-if="gameList.length > 0"
      class="flex flex-col sm:flex-row gap-4 p-4 bg-base-200 justify-between items-center"
    >
      <div class="text-sm text-base-content/70">
        Showing {{ Math.min(pageOffset + 1, gameList.length) }}-{{
          Math.min(pageOffset + pageSize, gameList.length)
        }}
        of {{ gameList.length }} games
      </div>

      <div class="join">
        <button
          class="join-item btn btn-sm"
          :disabled="page === 1"
          @click="pageDecrement"
          aria-label="Previous page"
        >
          «
        </button>
        <button class="join-item btn btn-sm">
          Page {{ page }}/{{ pageCount }}
        </button>
        <button
          class="join-item btn btn-sm"
          :disabled="page === pageCount"
          @click="pageIncrement"
          aria-label="Next page"
        >
          »
        </button>
      </div>
    </div>
  </div>

  <!-- Context Menu -->
  <ContextMenu
    :visible="contextMenu.visible"
    :x="contextMenu.x"
    :y="contextMenu.y"
    :items="contextMenuItems"
    @item-click="handleContextMenuClick"
    @close="hideContextMenu"
  />
</template>

<script setup lang="ts">
import {
  PhArrowSquareOut,
  PhCopy,
  PhFilePlus,
  PhPencilSimpleLine,
  PhShare,
  PhSortAscending,
  PhSortDescending,
  PhTrash,
} from "@phosphor-icons/vue";
import { computed, ref, watch } from "vue";
import { useGlobalStore } from "../../stores";
import { ContextMenu, type MenuItem } from "../Layout/ContextMenu";

// Types
interface ExplorerGame {
  id: number;
  date?: string;
  white_player: { name: string };
  black_player: { name: string };
  result: string;
}

const globalStore = useGlobalStore();
const uiStore = globalStore.uiStore;
const gamesStore = globalStore.gamesStore;

// Reactive state
const currentSort = ref("date_desc");
const currentFilter = ref("all");
const searchQuery = ref("");
const page = ref(1);
const isLoading = ref(false);
const error = ref<string | null>(null);

// Context menu state
const contextMenu = ref({
  visible: false,
  x: 0,
  y: 0,
  gameId: 0,
});

// Constants
const CURRENT_USER_NAME = "You"; // TODO: Get from user store
const PAGE_SIZE = 20; // Fixed page size - layout container handles overflow

// Filter function
function setFilter(filterType: string) {
  currentFilter.value = filterType;
  page.value = 1; // Reset to first page when filter changes
}

// Get filter label for display
const getFilterLabel = (filter: string): string => {
  const labels: Record<string, string> = {
    my_games: "My Games",
    wins: "Wins",
    losses: "Losses",
    draws: "Draws",
  };
  return labels[filter] || filter;
};

// Format date for display
const formatDate = (dateString?: string): string => {
  if (!dateString) return "—";

  try {
    const date = new Date(dateString);
    if (Number.isNaN(date.getTime())) return "—";

    return date.toLocaleDateString(undefined, {
      year: "numeric",
      month: "numeric",
      day: "numeric",
    });
  } catch {
    return "—";
  }
};

// Computed property for filtered and sorted games
const gameList = computed(() => {
  try {
    let games: ExplorerGame[] = [...(globalStore.explorerGames || [])];

    // Apply search filter
    if (searchQuery.value.trim()) {
      const query = searchQuery.value.toLowerCase().trim();
      games = games.filter(
        (game) =>
          game.white_player.name.toLowerCase().includes(query) ||
          game.black_player.name.toLowerCase().includes(query)
      );
    }

    // Apply category filter
    if (currentFilter.value !== "all") {
      games = games.filter((game) => {
        switch (currentFilter.value) {
          case "my_games":
            return (
              game.white_player.name === CURRENT_USER_NAME ||
              game.black_player.name === CURRENT_USER_NAME
            );
          case "wins":
            return (
              (game.white_player.name === CURRENT_USER_NAME &&
                game.result === "1-0") ||
              (game.black_player.name === CURRENT_USER_NAME &&
                game.result === "0-1")
            );
          case "losses":
            return (
              (game.white_player.name === CURRENT_USER_NAME &&
                game.result === "0-1") ||
              (game.black_player.name === CURRENT_USER_NAME &&
                game.result === "1-0")
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
          const dateA = new Date(a.date || "").getTime() || 0;
          const dateB = new Date(b.date || "").getTime() || 0;
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
  } catch (err) {
    console.error("Error processing games:", err);
    error.value = "Error loading games";
    return [];
  }
});

// Fixed page size - no dynamic calculation needed
const pageSize = computed(() => PAGE_SIZE);

const pageCount = computed(() =>
  Math.max(1, Math.ceil(gameList.value.length / pageSize.value))
);

const pageOffset = computed(() => (page.value - 1) * pageSize.value);

const paginatedGames = computed(() =>
  gameList.value.slice(pageOffset.value, pageOffset.value + pageSize.value)
);

// Context menu items
const contextMenuItems = computed((): MenuItem[] => [
  {
    id: "open",
    label: "Open Game",
    icon: PhFilePlus,
  },
  {
    id: "open-new-board",
    label: "Open in New Board",
    icon: PhArrowSquareOut,
  },
  {
    id: "divider",
    label: "",
    type: "divider",
  },
  {
    id: "copy",
    label: "Copy Game",
    icon: PhCopy,
  },
  {
    id: "export",
    label: "Export Game",
    icon: PhShare,
  },
  {
    id: "divider-2",
    label: "",
    type: "divider",
  },
  {
    id: "edit",
    label: "Edit Game",
    icon: PhPencilSimpleLine,
  },
  {
    id: "delete",
    label: "Delete Game",
    icon: PhTrash,
    type: "destructive",
  },
]);

// Pagination controls
const pageDecrement = () => {
  page.value = Math.max(1, page.value - 1);
};

const pageIncrement = () => {
  page.value = Math.min(pageCount.value, page.value + 1);
};

// Reset page when filters change
watch([currentFilter, currentSort, searchQuery], () => {
  page.value = 1;
});

// Ensure page is valid when game list changes
watch(pageCount, (newPageCount) => {
  if (page.value > newPageCount) {
    page.value = Math.max(1, newPageCount);
  }
});

const openGame = (gameId: number) => {
  try {
    const activeBoardId = uiStore.activeBoardId;
    console.debug(`Opening game ${gameId} in board ${activeBoardId}`);
    gamesStore.openGame(gameId, activeBoardId);
  } catch (err) {
    console.error("Error opening game:", err);
    error.value = "Error opening game";
  }
};

const showContextMenu = (event: MouseEvent, gameId: number) => {
  contextMenu.value = {
    visible: true,
    x: event.clientX,
    y: event.clientY,
    gameId,
  };
};

const hideContextMenu = () => {
  contextMenu.value.visible = false;
};

const setSort = (sortType: string) => {
  if (sortType === "date") {
    currentSort.value =
      currentSort.value === "date_desc" ? "date_asc" : "date_desc";
  } else {
    currentSort.value = sortType;
  }
};

// Handle context menu item clicks
const handleContextMenuClick = (itemId: string) => {
  const gameId = contextMenu.value.gameId;

  switch (itemId) {
    case "open":
      openGame(gameId);
      break;
    case "open-new-board":
      // TODO: Implement opening in new board
      console.log("Open in new board:", gameId);
      break;
    case "copy":
      // TODO: Implement copy game
      console.log("Copy game:", gameId);
      break;
    case "export":
      // TODO: Implement export game
      console.log("Export game:", gameId);
      break;
    case "delete":
      // TODO: Implement delete game
      console.log("Delete game:", gameId);
      break;
  }
};
</script>

<style scoped>
/* Focus styles for accessibility - not available in Tailwind */
tr[tabindex]:focus {
  outline: 2px solid hsl(var(--p));
  outline-offset: -2px;
}

/* Improve mobile responsiveness */
@media (max-width: 640px) {
  .table th,
  .table td {
    padding: 0.5rem 0.25rem;
    font-size: 0.875rem;
  }
}
</style>
