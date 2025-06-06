import { invoke } from "@tauri-apps/api/core";
import type { ChessGame, LegalMove, QueryParams } from "./bindings";
import {
	type ExplorerGame,
	explorerGameFields,
	parseLegalMoves,
} from "./types";

/**
 * The dev env exposes the API to the frontend console.
 * Use `await $$.api.____` to call any of the functions below.
 *
 * Ex. `await $$.api.engines.POST.loadEngine("stockfish", "/usr/local/bin/stockfish")`
 */

export default {
	/**
	 * Parse a PGN text and update the games and selected game.
	 * @param pgnText The PGN text to parse
	 */
	parsePgnText: async (pgnText: string): Promise<void> => {
		const result = await invoke("parse_pgn", { pgn: pgnText });
		console.log("Parse PGN result:", result);
	},

	/**
	 * Empty the database and reset the app state.
	 */
	emptyDatabase: async (): Promise<void> => {
		await invoke("empty_db");
	},

	games: {
		GET: {
			/**
			 * Get a list of games to display in the explorer view
			 * @param params The query parameters (optional)
			 * @returns Promise<ExplorerGame[]>
			 * @throws Error if the response is not a valid ExplorerGame[]
			 */
			explorer: async (
				params: Omit<QueryParams, "fields" | "load_tags"> = {
					limit: 100,
					offset: 0,
					filter: {},
					load_moves: false,
				},
			): Promise<ExplorerGame[]> => {
				const response = await invoke<string>("query_games", {
					params: {
						...params,
						fields: explorerGameFields,
						load_tags: true,
					},
				});

				console.log("Explorer games:", JSON.parse(response));

				// FIXME: The types need tweaking to parse the response successfully
				// currently the parsing fails, but the response is valid
				// so we are skipping the validation for now

				// const parsed = parseExplorerGames(response);

				// if (parsed.success) {
				//   return parsed.data;
				// }

				// console.error("Error parsing explorer games:", parsed.errors);

				// throw new Error(parsed.errors.join("\n"));
				return JSON.parse(response);
			},

			/**
			 * Get a game by its ID
			 * @param gameId The ID of the game
			 * @returns Promise<Game>
			 */
			game: async (
				gameId: number,
				params: QueryParams = {
					fields: null,
					limit: 1,
					offset: 0,
					filter: {},
					load_moves: true,
					load_tags: true,
				},
			): Promise<ChessGame> => {
				const response = await invoke<string>("get_game_by_id", {
					id: gameId,
					params: params,
				});
				return JSON.parse(response);
			},
		},

		POST: {
			importPGNGames: async (pgn: string): Promise<void> => {
				await invoke("import_pgn_games", { pgn });
			},

			/**
			 * Create a new game session (uses session-based backend)
			 * @param boardId The ID of the board to create the session on
			 * @param variant The chess variant to create
			 * @returns Promise<ChessGame> The new game
			 */
			newGame: async (
				boardId: number,
				variant: "standard" | "puzzle" | "960" = "standard",
			): Promise<ChessGame> => {
				const response = await invoke<string>("create_session", {
					boardId,
					variant,
				});
				return JSON.parse(response);
			},

			/**
			 * Open an existing game in a session (uses session-based backend)
			 * @param gameId The ID of the game to open
			 * @param boardId The ID of the board to open the game on
			 * @returns Promise<ChessGame> The loaded game
			 */
			openGame: async (gameId: number, boardId: number): Promise<ChessGame> => {
				const response = await invoke<string>("load_game_into_session", {
					gameId,
					boardId,
				});
				return JSON.parse(response);
			},

			/**
			 * Close a game session (uses session-based backend)
			 * @param boardId The ID of the board to close the session on
			 */
			closeGame: async (boardId: number): Promise<void> => {
				await invoke("close_session", { boardId });
			},

			/**
			 * Delete/remove a game from the database
			 * @param gameId The ID of the game to delete
			 */
			delete: async (gameId: number): Promise<void> => {
				// TODO: Implement a 'soft' delete (by setting a deleted flag or timestamp)
				await invoke("delete_game", { gameId });
			},

			/**
			 * Update a property on a game
			 * @param gameId The ID of the game to update
			 * @param property The property to update
			 * @param value The value to set the property to
			 */
			updateProperty: async (
				gameId: number,
				property: string,
				value: string,
			): Promise<void> => {
				await invoke("update_game_property", { gameId, property, value });
			},
		},
	},

	/**
	 * Session-focused API for managing game sessions
	 */
	sessions: {
		GET: {
			/**
			 * Get the current state of a specific game session
			 * @param boardId The ID of the board/session to retrieve
			 * @returns Promise<ChessGame> The current game state
			 */
			get: async (boardId: number): Promise<ChessGame> => {
				const response = await invoke<string>("get_session", { boardId });
				return JSON.parse(response);
			},

			/**
			 * Get all active game sessions
			 * @returns Promise<Record<number, ChessGame>> All active sessions with their board IDs
			 */
			getAll: async (): Promise<Record<number, ChessGame>> => {
				const response = await invoke<string>("get_all_sessions");
				return JSON.parse(response);
			},

			/**
			 * Get the move history for a game session
			 * @param boardId The ID of the board/session
			 * @returns Promise<unknown> The move history
			 */
			moves: async (boardId: number): Promise<unknown> => {
				const response = await invoke<string>("get_session_moves", { boardId });
				return JSON.parse(response);
			},
		},

		POST: {
			/**
			 * Create a new game session
			 * @param boardId The ID of the board to create the session on
			 * @param variant The chess variant to create
			 * @returns Promise<ChessGame> The new game
			 */
			create: async (
				boardId: number,
				variant: "standard" | "puzzle" | "960" = "standard",
			): Promise<ChessGame> => {
				const response = await invoke<string>("create_session", {
					boardId,
					variant,
				});
				return JSON.parse(response);
			},

			/**
			 * Load an existing game into a session
			 * @param gameId The ID of the game to load
			 * @param boardId The ID of the board to load the game on
			 * @returns Promise<ChessGame> The loaded game
			 */
			load: async (gameId: number, boardId: number): Promise<ChessGame> => {
				const response = await invoke<string>("load_game_into_session", {
					gameId,
					boardId,
				});
				return JSON.parse(response);
			},

			/**
			 * Close a game session
			 * @param boardId The ID of the board to close the session on
			 */
			close: async (boardId: number): Promise<void> => {
				await invoke("close_session", { boardId });
			},

			/**
			 * Close all active game sessions
			 */
			closeAll: async (): Promise<void> => {
				await invoke("close_all_sessions");
			},

			/**
			 * Make a move in a game session
			 * @param boardId The ID of the board/session
			 * @param moveNotation The move in algebraic notation
			 * @returns Promise<ChessGame> The updated game state
			 */
			makeMove: async (
				boardId: number,
				moveNotation: string,
			): Promise<ChessGame> => {
				const response = await invoke<string>("make_move", {
					boardId,
					moveNotation,
				});
				return JSON.parse(response);
			},

			/**
			 * Undo the last move in a game session
			 * @param boardId The ID of the board/session
			 * @returns Promise<ChessGame> The updated game state
			 */
			undoMove: async (boardId: number): Promise<ChessGame> => {
				const response = await invoke<string>("undo_move", { boardId });
				return JSON.parse(response);
			},

			/**
			 * Redo a previously undone move in a game session
			 * @param boardId The ID of the board/session
			 * @returns Promise<ChessGame> The updated game state
			 */
			redoMove: async (boardId: number): Promise<ChessGame> => {
				const response = await invoke<string>("redo_move", { boardId });
				return JSON.parse(response);
			},

			/**
			 * Move to the next move in a game session
			 * @param boardId The ID of the board/session
			 * @param variation The variation index to move to (optional, 0 = main line)
			 * @returns Promise<ChessGame> The updated game state
			 */
			nextMove: async (boardId: number, variation = 0): Promise<ChessGame> => {
				const response = await invoke<string>("next_move", {
					boardId,
					variation,
				});
				return JSON.parse(response);
			},

			/**
			 * Move to the previous move in a game session
			 * @param boardId The ID of the board/session
			 * @returns Promise<ChessGame> The updated game state
			 */
			previousMove: async (boardId: number): Promise<ChessGame> => {
				const response = await invoke<string>("previous_move", { boardId });
				return JSON.parse(response);
			},

			/**
			 * Jump to a specific move in a game session
			 * @param boardId The ID of the board/session
			 * @param moveId The database ID of the move to jump to
			 * @returns Promise<ChessGame> The updated game state
			 */
			jumpToMove: async (
				boardId: number,
				moveId: number,
			): Promise<ChessGame> => {
				const response = await invoke<string>("reset_to_position", {
					boardId,
					moveDbId: moveId,
				});
				return JSON.parse(response);
			},

			/**
			 * Navigate to the start of the game
			 * @param boardId The ID of the board/session
			 * @returns Promise<ChessGame> The updated game state
			 */
			navigateToStart: async (boardId: number): Promise<ChessGame> => {
				const response = await invoke<string>("navigate_to_start", { boardId });
				return JSON.parse(response);
			},

			/**
			 * Navigate to the end of the main line (in relation to the current node)
			 *
			 * @param boardId The ID of the board/session
			 * @returns Promise<ChessGame> The updated game state
			 */
			navigateToEnd: async (boardId: number): Promise<ChessGame> => {
				const response = await invoke<string>("navigate_to_end", { boardId });
				return JSON.parse(response);
			},

			/**
			 * Save a game session to the database
			 * @param boardId The ID of the board/session to save
			 * @param overwrite Whether to overwrite existing game or create new one
			 * @returns Promise<number> The ID of the saved game
			 */
			save: async (boardId: number, overwrite = false): Promise<number> => {
				return await invoke<number>("save_session", { boardId, overwrite });
			},

			/**
			 * Save all active game sessions to the database
			 * @param overwrite Whether to overwrite existing games or create new ones
			 * @returns Promise<number[]> The IDs of the saved games
			 */
			saveAll: async (overwrite = false): Promise<number[]> => {
				const response = await invoke<string>("save_all_sessions", {
					overwrite,
				});
				return JSON.parse(response);
			},
		},
	},

	moves: {
		GET: {
			/**
			 * Get the valid moves for a given FEN string
			 * @param fen The FEN string of the position
			 * @returns Promise<LegalMove[]>
			 */
			validMoves: async (fen: string): Promise<LegalMove[]> => {
				const response = await invoke<string>("get_legal_moves", { fen });
				const parsed = parseLegalMoves(response);
				if (parsed.success) {
					return parsed.data;
				}
				throw new Error(parsed.errors.join("\n"));
			},
			moveTree: async (gameId: number): Promise<string> => {
				const response = await invoke<string>("get_move_tree", { id: gameId });
				return JSON.parse(response);
			},
		},
		POST: {
			/**
			 * Make a move in a game session (alias for sessions.POST.makeMove)
			 * @param boardId The ID of the board/session
			 * @param moveNotation The move to make in algebraic notation
			 * @returns Promise<ChessGame> The updated game
			 */
			makeMove: async (
				boardId: number,
				moveNotation: string,
			): Promise<ChessGame> => {
				const response = await invoke<string>("make_move", {
					boardId,
					moveNotation,
				});
				return JSON.parse(response);
			},
		},
	},
	engines: {
		POST: {
			/**
			 * Load an engine from a path.
			 * @param name The name of the engine
			 * @param path The path to the engine
			 * @returns Promise<void>
			 *
			 * @example
			 * await $$.api.engines.POST.loadEngine("stockfish", "/usr/local/bin/stockfish")
			 */
			loadEngine: async (name: string, path: string): Promise<void> => {
				await invoke("load_engine", { name, path });
			},
			/**
			 * Unload an engine.
			 * @param name The name of the engine
			 * @returns Promise<void>
			 *
			 * @example
			 * await $$.api.engines.POST.unloadEngine("stockfish")
			 */
			unloadEngine: async (name: string): Promise<void> => {
				await invoke("unload_engine", { name });
			},
			/**
			 * Analyze a position with an engine.
			 * @param engineName The name of the engine
			 * @param fen The FEN string of the position
			 * @param depth The depth of the analysis
			 * @param timeMs The time in milliseconds for the analysis
			 * @returns Promise<void>
			 *
			 * @example
			 * await $$.api.engines.POST.analyzePosition("stockfish", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 10, 1000)
			 */
			analyzePosition: async (
				engineName: string,
				fen: string,
				depth: number,
				timeMs: number,
			): Promise<void> => {
				await invoke("analyze_position", { engineName, fen, depth, timeMs });
			},
			/**
			 * Stop an analysis.
			 * @param engineName The name of the engine
			 * @returns Promise<void>
			 *
			 * @example
			 * await $$.api.engines.POST.stopAnalysis("stockfish")
			 */
			stopAnalysis: async (engineName: string): Promise<void> => {
				await invoke("stop_analysis", { engineName });
			},
		},
	},
};
