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
					<span
						class="ml-1 text-xs opacity-70"
						v-if="currentFilter !== 'all'"
					>
						 ({{ getFilterLabel(currentFilter) }})
					</span>

				</label>

				<ul
					tabindex="0"
					class="dropdown-content z-10 menu p-2 shadow-sm bg-base-300 rounded-box w-52"
					role="menu"
				>

					<li role="none">

						<a
							@click="setFilter('all')"
							role="menuitem"
						>
							 All Games
						</a>

					</li>

					<li role="none">

						<a
							@click="setFilter('favorites')"
							role="menuitem"
						>
							 My Games
						</a>

					</li>

					<li role="none">

						<a
							@click="setFilter('tags', ['win'])"
							role="menuitem"
						>
							 Wins
						</a>

					</li>

					<li role="none">

						<a
							@click="setFilter('tags', ['loss'])"
							role="menuitem"
						>
							 Losses
						</a>

					</li>

					<li role="none">

						<a
							@click="setFilter('tags', ['draw'])"
							role="menuitem"
						>
							 Draws
						</a>

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

		<div
			v-if="isLoading"
			class="flex-1 flex justify-center items-center p-8"
		>

			<span class="loading loading-spinner loading-lg"></span>

		</div>

		<!-- Error state -->

		<div
			v-else-if="error"
			class="flex-1 flex justify-center items-center p-8"
		>

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

		<div
			v-else
			class="flex-1 min-h-0"
		>

			<table
				class="table table-sm table-pin-rows table-pin-cols"
				role="table"
			>

				<!-- Table header -->

				<thead>

					<tr role="row">

						<th
							scope="col"
							@click="setSort('date')"
							class="cursor-pointer hover:bg-info/10 flex gap-1 justify-between items-center"
							:class="{
								'bg-info/50':
									currentSort.field === 'date' && currentSort.order === 'desc',
							}"
						>
							 Date
							<PhSortAscending
								v-if="
									currentSort.field === 'date' && currentSort.order === 'asc'
								"
								size="16"
							/>

							<PhSortDescending
								v-if="
									currentSort.field === 'date' && currentSort.order === 'desc'
								"
								size="16"
							/>

						</th>

						<th
							scope="col"
							@click="setSort('white')"
							class="cursor-pointer hover:bg-info/10"
							:class="{
								'bg-info/50': currentSort.field === 'white',
							}"
						>
							 White
						</th>

						<th
							scope="col"
							@click="setSort('black')"
							class="cursor-pointer hover:bg-info/10"
							:class="{
								'bg-info/50': currentSort.field === 'black',
							}"
						>
							 Black
						</th>

						<th
							scope="col"
							@click="setSort('result')"
							class="cursor-pointer hover:bg-info/10"
							:class="{
								'bg-info/50': currentSort.field === 'result',
							}"
						>
							 Result
						</th>

					</tr>

				</thead>

				<!-- Table body -->

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

						<td @dblclick="startEdit(game.id, 'date', game.date ?? '')">

							<span
								v-if="
									editingField?.gameId !== game.id ||
									editingField?.field !== 'date'
								"
							>
								 {{ formatDate(game.date) }}
							</span>

							<input
								v-else
								v-model="editingField.inputValue"
								name="game-date"
								class="input input-xs input-neutral w-full text-base-content"
								autofocus
								@blur="stopEdit"
								@keydown.enter="stopEdit"
								@keydown.esc="stopEdit"
							/>

						</td>

						<td
							@dblclick="
								startEdit(game.id, 'white_player_name', game.white_player.name)
							"
						>

							<span
								v-if="
									editingField?.gameId !== game.id ||
									editingField?.field !== 'white_player_name'
								"
							>
								 {{ game.white_player.name }}
							</span>

							<input
								v-else
								v-model="editingField.inputValue"
								name="game-white-player-name"
								class="input input-xs input-neutral w-full text-base-content"
								@blur="stopEdit"
								@keydown.enter="stopEdit"
								@keydown.esc="stopEdit"
							/>

						</td>

						<td
							@dblclick="
								startEdit(game.id, 'black_player_name', game.black_player.name)
							"
						>

							<span
								v-if="
									editingField?.gameId !== game.id ||
									editingField?.field !== 'black_player_name'
								"
							>
								 {{ game.black_player.name }}
							</span>

							<input
								v-else
								v-model="editingField.inputValue"
								name="game-black-player-name"
								class="input input-xs input-neutral w-full text-base-content"
								@blur="stopEdit"
								@keydown.enter="stopEdit"
								@keydown.esc="stopEdit"
							/>

						</td>

						<td @dblclick="startEdit(game.id, 'result', game.result)">

							<span
								v-if="
									editingField?.gameId !== game.id ||
									editingField?.field !== 'result'
								"
								class="badge badge-xs float-right break-keep"
								:class="{
									'badge-success': game.result === '1-0',
									'badge-error': game.result === '0-1',
									'badge-warning': game.result === '1/2-1/2',
									'badge-neutral': game.result === '*',
								}"
							>
								 {{ game.result }}
							</span>

							<select
								v-else
								v-model="editingField.inputValue"
								name="game-result"
								class="select select-xs select-neutral w-full text-base-content"
								@blur="stopEdit"
								@keydown.enter="stopEdit"
								@keydown.esc="stopEdit"
							>

								<option
									value="*"
									:selected="game.result === '*'"
								>
									 *
								</option>

								<option
									value="1-0"
									:selected="game.result === '1-0'"
								>
									 1-0
								</option>

								<option
									value="0-1"
									:selected="game.result === '0-1'"
								>
									 0-1
								</option>

								<option
									value="1/2-1/2"
									:selected="game.result === '1/2-1/2'"
								>
									 1/2-1/2
								</option>

							</select>

						</td>

					</tr>

				</tbody>

			</table>

		</div>

		<!-- Pagination -->

		<div
			v-if="gameList.length > 0"
			class="flex flex-col sm:flex-row gap-4 p-4 bg-base-200 justify-between items-center"
		>

			<div class="text-sm text-base-content/70">
				 Showing {{ Math.min(pageOffset + 1, gameList.length) }}-{{
					Math.min(pageOffset + pageSize, gameList.length)
				}} of {{ gameList.length }} games
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
	PhFilePlus,
	PhPencilSimpleLine,
	PhShare,
	PhSortAscending,
	PhSortDescending,
	PhTrash,
} from "@phosphor-icons/vue";
import { computed, inject, ref, watch } from "vue";
import {
	ExplorerGame,
	FilterOption,
	SortConfig,
	SortOption,
} from "../../shared/types";
import { ContextMenu, type MenuItem } from "../Layout/ContextMenu";
import {
	IImportExportService,
	ImportExportServiceKey,
} from "../../composables/useInjection";

const props = defineProps<{
	games: ExplorerGame[];
	isLoading: boolean;
	error: string | null;
}>();

const emit = defineEmits<{
	"open-game": [payload: { gameId: number; newBoard: boolean }];
	"delete-game": [gameId: number];
	"update-game-property": [
		payload: { gameId: number; field: string; value: string },
	];
	"refresh-games": [];
}>();

const importExportService = inject(
	ImportExportServiceKey,
) as IImportExportService;

const { searchGames, filterGames, sortGames } = importExportService;

// Reactive state
const currentSort = ref<SortConfig>({
	field: "date",
	order: "desc",
});
const currentFilter = ref<FilterOption>("all");
const currentFilterTags = ref<string[]>([]);
const searchQuery = ref("");
const editingField = ref<{
	gameId: number;
	field: string;
	inputValue: string;
} | null>(null);
const page = ref(1);

// Context menu state
const contextMenu = ref({
	visible: false,
	x: 0,
	y: 0,
	gameId: 0,
});

// Constants
// const CURRENT_USER_NAME = "You"; // TODO: Get from user store
// TODO: Make this dynamic
const PAGE_SIZE = 10; // Fixed page size - layout container handles overflow

// Filter function
function setFilter(filterType: FilterOption, filterTags: string[] = []) {
	currentFilter.value = filterType;
	currentFilterTags.value = filterTags;
	page.value = 1; // Reset to first page when filter changes
}

// Get filter label for display
const getFilterLabel = (filter: FilterOption): string => {
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
		// Date string is in "YYYY.MM.DD" format
		// with "??" for unknown parts
		// ex. 2025.05.23, 2025.??.??, 2025.05.??
		const [year, month, day] = dateString.split(".");

		// TODO: Support other date formats
		// could be tracked in settings for now,
		// at some point it should be connected to the user's locale

		dateString = `${year}/${month}/${day}`;
	} catch {
		return "—";
	}

	return dateString;
};

// Computed property for filtered and sorted games
const gameList = computed(() => {
	let games: ExplorerGame[] = [...(props.games || [])];

	if (!importExportService || !searchGames || !filterGames || !sortGames) {
		console.warn(
			"ImportExportService not provided correctly. Some features will not be available.",
		);
		return games;
	}

	games = searchGames(games, searchQuery.value);

	games = filterGames(games, currentFilter.value, currentFilterTags.value);

	games = sortGames(games, currentSort.value.field, currentSort.value.order);

	return games;
});

// Fixed page size - no dynamic calculation needed
const pageSize = computed(() => PAGE_SIZE);

const pageCount = computed(() =>
	Math.max(1, Math.ceil(gameList.value.length / pageSize.value)),
);

const pageOffset = computed(() => (page.value - 1) * pageSize.value);

const paginatedGames = computed(() =>
	gameList.value.slice(pageOffset.value, pageOffset.value + pageSize.value),
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

const startEdit = (gameId: number, field: string, inputValue: string) => {
	editingField.value = { gameId, field, inputValue };
};

const stopEdit = async () => {
	// Save the value to the game
	if (editingField.value) {
		emit("update-game-property", {
			gameId: editingField.value.gameId,
			field: editingField.value.field,
			value: editingField.value.inputValue,
		});
	}
	editingField.value = null;
};

const openGame = (gameId: number) => {
	emit("open-game", { gameId, newBoard: false });
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

const setSort = (sortType: SortOption) => {
	if (sortType === "date") {
		currentSort.value =
			currentSort.value.order === "desc"
				? { field: "date", order: "asc" }
				: { field: "date", order: "desc" };
	} else {
		currentSort.value = { field: sortType, order: "desc" };
	}
};

// Handle context menu item clicks
const handleContextMenuClick = async (itemId: string) => {
	const gameId = contextMenu.value.gameId;

	switch (itemId) {
		case "open":
			openGame(gameId);
			break;
		case "open-new-board":
			// Create new active board
			emit("open-game", { gameId, newBoard: true });
			break;
		case "edit":
			break;
		case "export":
			// TODO: Implement export game
			console.log("Export game:", gameId);
			break;
		case "delete":
			emit("delete-game", gameId);
			// refresh the game list
			emit("refresh-games");
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

