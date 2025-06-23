import type { InjectionKey } from "vue";
import type {
	ActiveGameState,
	AlertToast,
	ExplorerGame,
	FilterOption,
	OperationResult,
	SortOption,
} from "../shared/types";
import {
	UITheme,
	LightUITheme,
	DarkUITheme,
	BoardTheme,
} from "../shared/themes";

// Define interfaces for the services we want to inject
export interface IGlobalStore {
	importPGNGames(pgn: string): Promise<void>;
	updateGameProperty(
		gameId: number,
		property: string,
		value: string,
	): Promise<void>;
	fetchExplorerGames(): Promise<ExplorerGame[]>;
	explorerGames: ExplorerGame[];
}

// UiStoreKey
export interface IUiStore {
	addAlert(alert: Omit<AlertToast, "key"> & { key?: string }): void;
	createNewBoard(): number;

	activeBoardId: number;
	visibleGameHeaders: string[];
	theme: UITheme;
	defaultLightTheme: LightUITheme;
	defaultDarkTheme: DarkUITheme;
	boardTheme: BoardTheme;
}

// GamesStoreKey
export interface IGamesStore {
	activeGameMap: Map<number, ActiveGameState>;
	openGame(gameId: number, boardId: number): Promise<ActiveGameState | null>;
	deleteGame(gameId: number): Promise<boolean>;
}

export interface IImportExportService {
	validatePGNFormat(pgn: string): { isValid: boolean; error?: string };
	importPGNGames(pgn: string): Promise<OperationResult>;
	fetchExplorerGames(): Promise<ExplorerGame[]>;
	searchGames(games: ExplorerGame[], query: string): ExplorerGame[];
	filterGames(
		games: ExplorerGame[],
		filter: FilterOption,
		filterTags: string[],
	): ExplorerGame[];
	sortGames(
		games: ExplorerGame[],
		sortType: SortOption,
		sortOrder: "asc" | "desc",
	): ExplorerGame[];
}

// Define injection keys with proper typing
export const GlobalStoreKey: InjectionKey<IGlobalStore> = Symbol("GlobalStore");
export const ImportExportServiceKey: InjectionKey<IImportExportService> =
	Symbol("ImportExportService");
export const UiStoreKey: InjectionKey<IUiStore> = Symbol("UiStore");
export const GamesStoreKey: InjectionKey<IGamesStore> = Symbol("GamesStore");

// Helper function to create a mock implementation
export const createMockGlobalStore = (): IGlobalStore => ({
	importPGNGames: async (pgn: string) => {
		console.log("Mock: Would import PGN:", pgn);
		await new Promise((resolve) => setTimeout(resolve, 1000));
	},
	updateGameProperty: async (
		gameId: number,
		property: string,
		value: string,
	) => {
		console.log("Mock: Would update game property:", gameId, property, value);
		await new Promise((resolve) => setTimeout(resolve, 1000));
	},
	explorerGames: [
		{
			id: 1,
			white_player: {
				id: 1,
				name: "Player 1",
				elo: 1000,
				country: "USA",
			},
			black_player: {
				id: 2,
				name: "Player 2",
				elo: 1000,
				country: "USA",
			},
			tournament: {
				id: 1,
				name: "Tournament 1",
				tournament_type: "Round Robin",
				time_control: "10+0",
				start_date: "2021-01-01",
				end_date: "2021-01-01",
				location: "USA",
			},
			opening: {
				id: 1,
				name: "Opening 1",
				eco: "A00",
				variation:
					"1. e4 e5 2. Nf3 Nc6 3. Bc4 Bc5 4. b4 Bxb4 5. c3 Ba5 6. d4 exd4 7. O-O d3 8. Qb3 Qb6 9. Rfe1 Qxb2 10. Nbd2 Qc3 11. Nf1 Qd2 12. Nc1 Qc3 13. Nb3 Qb2",
			},
			result: "1-0",
			fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
			variant: "standard",
			pgn: "1. e4 e5 2. Nf3 Nc6 3. Bc4 Bc5 4. b4 Bxb4 5. c3 Ba5 6. d4 exd4 7. O-O d3 8. Qb3 Qb6 9. Rfe1 Qxb2 10. Nbd2 Qc3 11. Nf1 Qd2 12. Nc1 Qc3 13. Nb3 Qb2",
			tags: ["A test tag", "Another test tag"],
			round: 1,
			date: "2021-01-01",
			headers: [
				{
					id: 1,
					game_id: 1,
					name: "Event",
					value: "Tournament 1",
				},
			],
			move_tree: {
				game_id: 1,
				nodes: [],
			},
		},
	],
	fetchExplorerGames: async () => {
		console.log("Mock: Would fetch explorer games");
		await new Promise((resolve) => setTimeout(resolve, 1000));
		return [];
	},
});

export const createMockImportExportService = (): IImportExportService => ({
	validatePGNFormat: (pgn: string) => {
		if (!pgn || pgn.trim().length === 0) {
			return { isValid: false, error: "PGN content is empty" };
		}

		// Basic checks for PGN format
		const lines = pgn.split("\n");
		let hasGameMoves = false;
		let hasHeaders = false;

		for (const line of lines) {
			const trimmedLine = line.trim();

			// Check for PGN headers
			if (trimmedLine.startsWith("[") && trimmedLine.endsWith("]")) {
				hasHeaders = true;
			}

			// Check for move notation (simplified check)
			if (trimmedLine.match(/^\d+\.?\s+[a-zA-Z]/)) {
				hasGameMoves = true;
			}
		}

		if (!hasHeaders && !hasGameMoves) {
			return { isValid: false, error: "No valid PGN headers or moves found" };
		}

		return { isValid: true };
	},
	importPGNGames: async (pgn: string) => {
		console.log("Mock service: Would import PGN:", pgn);
		await new Promise((resolve) => setTimeout(resolve, 500));
		return { success: true, data: undefined, error: undefined };
	},
	fetchExplorerGames: async () => {
		console.log("Mock: Would fetch explorer games");
		await new Promise((resolve) => setTimeout(resolve, 1000));
		return createMockGlobalStore().explorerGames;
	},

	searchGames: (games: ExplorerGame[], query: string) => {
		console.log("Mock: Would search games:", query);
		return games;
	},
	filterGames: (
		games: ExplorerGame[],
		filter: FilterOption,
		filterTags: string[],
	) => {
		console.log("Mock: Would filter games:", filter, filterTags);
		return games;
	},
	sortGames: (
		games: ExplorerGame[],
		sortType: SortOption,
		sortOrder: "asc" | "desc" = "desc",
	) => {
		console.log("Mock: Would sort games:", sortType, sortOrder);
		return games;
	},
});
