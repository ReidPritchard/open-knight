import { invoke } from "@tauri-apps/api/core";
import type { ChessGame, LegalMove, QueryParams } from "./bindings";
import { parseChessGame } from "./types";

/**
 * Wraps an API call in error handling
 *
 * @param operation - The operation to wrap
 * @param context - The context of the operation
 * @returns The result of the operation
 *
 * @throws ApplicationError
 */
const wrapAPICall = async <T>(
	operation: () => Promise<T>,
	context: string,
): Promise<T> => {
	try {
		return await operation();
	} catch (error) {
		const errorMessage = `API Error in ${context}: ${error}`;
		throw new Error(errorMessage);
	}
};

/**
 * API for interacting with the backend
 *
 * @description
 * Provides a type-safe interface for invoke calls.
 * Routes are grouped by category and named according to the operation they perform.
 *
 * @property games - Game/Database operations
 * @property board - Active game session operations
 * @property analysis - Chess analysis and engine operations
 * @property utils - Utility operations
 */
export const API = {
	/**
	 * Game/Database operations
	 *
	 * @property list - List all games in the database
	 * @property get - Get a game by id
	 * @property create - Create a new game
	 * @property delete - Delete a game from the database
	 * @property import - Import a PGN file
	 * @property update - Update a game property
	 */
	games: {
		list: (params: Partial<QueryParams> = {}) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("query_games", {
					params: {
						limit: 100,
						offset: 0,
						filter: {},
						load_moves: false,
						...params,
					},
				});
				return JSON.parse(response);
			}, "list games"),

		create: (
			boardId: number,
			variant: "standard" | "puzzle" | "960" = "standard",
		) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("create_session", {
					boardId,
					variant,
				});
				return JSON.parse(response) as ChessGame;
			}, "create game"),

		delete: (gameId: number) =>
			wrapAPICall<void>(
				() => invoke<void>("delete_game", { gameId }),
				`delete game ${gameId}`,
			),

		import: (pgn: string) =>
			wrapAPICall<void>(
				() => invoke<void>("import_pgn_games", { pgn }),
				"import PGN",
			),

		update: (gameId: number, property: string, value: string) =>
			wrapAPICall<void>(
				() => invoke<void>("update_game_property", { gameId, property, value }),
				`update game ${gameId}`,
			),
	},

	board: {
		open: (gameId: number, boardId: number) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("load_game_into_session", {
					gameId,
					boardId,
				});
				return JSON.parse(response) as ChessGame;
			}, `open game ${gameId} on board ${boardId}`),

		close: (boardId: number) =>
			wrapAPICall<void>(
				() => invoke<void>("close_session", { boardId }),
				`close board ${boardId}`,
			),

		getState: (boardId: number) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("get_session", { boardId });
				const game_result = parseChessGame(response);
				if (game_result.success) {
					return game_result.data;
				} else {
					throw new Error(game_result.errors.join(", "));
				}
			}, `get board ${boardId} state`),

		getAllStates: () =>
			wrapAPICall(async () => {
				const response = await invoke<string>("get_all_sessions");
				return JSON.parse(response) as Record<number, ChessGame>;
			}, "get all board states"),

		move: (boardId: number, notation: string) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("make_move", {
					boardId,
					moveNotation: notation,
				});
				return JSON.parse(response) as ChessGame;
			}, `make move ${notation} on board ${boardId}`),

		undo: (boardId: number) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("undo_move", { boardId });
				return JSON.parse(response) as ChessGame;
			}, `undo move on board ${boardId}`),

		redo: (boardId: number) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("redo_move", { boardId });
				return JSON.parse(response) as ChessGame;
			}, `redo move on board ${boardId}`),

		next: (boardId: number, variation = 0) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("next_move", {
					boardId,
					variation,
				});
				return JSON.parse(response) as ChessGame;
			}, `next move on board ${boardId}`),

		previous: (boardId: number) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("previous_move", { boardId });
				return JSON.parse(response) as ChessGame;
			}, `previous move on board ${boardId}`),

		jumpTo: (boardId: number, moveId: number) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("reset_to_position", {
					boardId,
					moveDbId: moveId,
				});
				return JSON.parse(response) as ChessGame;
			}, `jump to move ${moveId} on board ${boardId}`),

		toStart: (boardId: number) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("navigate_to_start", { boardId });
				return JSON.parse(response) as ChessGame;
			}, `navigate to start on board ${boardId}`),

		toEnd: (boardId: number) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("navigate_to_end", { boardId });
				return JSON.parse(response) as ChessGame;
			}, `navigate to end on board ${boardId}`),

		save: (boardId: number, overwrite = false) =>
			wrapAPICall(
				() => invoke<number>("save_session", { boardId, overwrite }),
				`save board ${boardId}`,
			),
	},

	analysis: {
		getValidMoves: (fen: string) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("get_legal_moves", { fen });
				// Parse with proper validation
				return JSON.parse(response) as LegalMove[];
			}, "get valid moves"),

		getMoveTree: (gameId: number) =>
			wrapAPICall(async () => {
				const response = await invoke<string>("get_move_tree", { id: gameId });
				return JSON.parse(response);
			}, `get move tree for game ${gameId}`),

		loadEngine: (name: string, path: string) =>
			wrapAPICall<void>(
				() => invoke<void>("load_engine", { name, path }),
				`load engine ${name}`,
			),

		unloadEngine: (name: string) =>
			wrapAPICall<void>(
				() => invoke<void>("unload_engine", { name }),
				`unload engine ${name}`,
			),

		analyze: (
			engineName: string,
			fen: string,
			options: { depth: number; timeMs: number },
		) =>
			wrapAPICall<void>(
				() =>
					invoke<void>("analyze_position", {
						engineName,
						fen,
						depth: options.depth,
						timeMs: options.timeMs,
					}),
				`analyze position with ${engineName}`,
			),

		analyzeGame: (engineName: string, gameId: number) =>
			wrapAPICall<void>(
				() => invoke<void>("analyze_game", { engineName, gameId }),
				`analyze game ${gameId} with ${engineName}`,
			),

		stopAnalysis: (engineName: string) =>
			wrapAPICall<void>(
				() => invoke<void>("stop_analysis", { engineName }),
				`stop analysis for ${engineName}`,
			),

		setEngineOption: (engineName: string, option: string, value: string) =>
			wrapAPICall<void>(
				() => invoke<void>("set_engine_option", { engineName, option, value }),
				`set engine option ${option}=${value} for ${engineName}`,
			),
	},

	utils: {
		parsePGN: (pgnText: string) =>
			wrapAPICall<void>(
				() => invoke<void>("parse_pgn", { pgn: pgnText }),
				"parse PGN",
			),

		emptyDatabase: () =>
			wrapAPICall<void>(() => invoke<void>("empty_db"), "empty database"),
	},
};

export default API;
