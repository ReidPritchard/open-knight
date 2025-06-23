<template>

	<Story title="Game Library">

		<Variant title="Default">

			<GameLibrary
				:games="games"
				:is-loading="isLoading"
				:error="error"
				@open-game="logEvent('open-game', $event)"
				@delete-game="logEvent('delete-game', $event)"
				@update-game-property="logEvent('update-game-property', $event)"
				@refresh-games="logEvent('refresh-games', $event)"
			/>

		</Variant>

	</Story>

</template>

<script setup lang="ts">
import { ref } from "vue";
import GameLibrary from "./GameLibrary.vue";
import type { ExplorerGame } from "../../shared/types";
import { logEvent } from "histoire/client";

const isLoading = ref(false);
const error = ref<string | null>(null);
const games = ref<ExplorerGame[]>([
	{
		id: 1,
		white_player: { id: 1, name: "Player 1", elo: 1800, country: null },
		black_player: { id: 2, name: "Player 2", elo: 1850, country: null },
		date: "2024.01.01",
		result: "1-0",
		tournament: null,
		opening: null,
		round: 1,
		tags: ["favorite"],
		headers: [],
		move_tree: {
			game_id: 0,
			nodes: [],
			root_id: undefined,
			current_node_id: undefined,
		},
		fen: null,
		variant: "standard",
		pgn: null,
	},
	{
		id: 2,
		white_player: {
			id: 3,
			name: "Unknown Game Date",
			elo: 1900,
			country: null,
		},
		black_player: { id: 1, name: "Player 1", elo: 1800, country: null },
		date: "",
		result: "1/2-1/2",
		tournament: null,
		opening: null,
		round: 2,
		tags: [],
		headers: [],
		move_tree: {
			game_id: 0,
			nodes: [],
			root_id: undefined,
			current_node_id: undefined,
		},
		fen: null,
		variant: "standard",
		pgn: null,
	},
	{
		id: 3,
		white_player: {
			id: 1,
			name: "Slightly Longer Name",
			elo: 1800,
			country: null,
		},
		black_player: {
			id: 2,
			name: "Even-Slightly Longer Name",
			elo: 1850,
			country: null,
		},
		date: "2024.01.03",
		result: "0-1",
		tournament: null,
		opening: null,
		round: 3,
		tags: [],
		headers: [],
		move_tree: {
			game_id: 0,
			nodes: [],
			root_id: undefined,
			current_node_id: undefined,
		},
		fen: null,
		variant: "standard",
		pgn: null,
	},
	{
		id: 4,
		white_player: {
			id: 1,
			name: "Player Name For An Ongoing Game",
			elo: 1800,
			country: null,
		},
		black_player: {
			id: 2,
			name: "Player Name For Another Ongoing Game",
			elo: 1850,
			country: null,
		},
		date: "2024.01.04",
		result: "*",
		tournament: null,
		opening: null,
		round: 4,
		tags: [],
		headers: [],
		move_tree: {
			game_id: 0,
			nodes: [],
			root_id: undefined,
			current_node_id: undefined,
		},
		fen: null,
		variant: "standard",
		pgn: null,
	},
]);
</script>

