import { defineStore } from "pinia";
import { useError } from "../composables/useError";
import {
	closeGameSession,
	createGameSession,
	deleteGame as deleteGameService,
	getCurrentMove,
	getCurrentPosition,
	getTurnFromFen,
	getValidMoves,
	hasMultipleVariations,
	jumpToMove,
	loadGameSession,
	makeMove,
	navigateToEnd,
	navigateToNextMove,
	navigateToPreviousMove,
	navigateToStart,
	refreshGameState,
	saveGameSession,
} from "../services/GameService";
import type { ChessGame } from "../shared/bindings";

interface ActiveGameState {
	id: number;
	game: ChessGame;

	// UI state
	hideEvaluationBar: boolean;
	hideBestMove: boolean;
	hideThreats: boolean;

	// Loading states
	isLoading: boolean;
	error: string | null;
}

/**
 * A store for managing the states of ALL open games using session-focused API
 */
export const useGamesStore = defineStore("games", {
	state: () => ({
		activeGameMap: new Map<number, ActiveGameState>(),
	}),

	getters: {
		getBoardState: (state) => (boardId: number) =>
			state.activeGameMap.get(boardId),

		getCurrentPosition: (state) => (boardId: number) => {
			const gameState = state.activeGameMap.get(boardId);
			if (!gameState) return null;
			return getCurrentPosition(gameState.game);
		},

		getCurrentMove: (state) => (boardId: number) => {
			const gameState = state.activeGameMap.get(boardId);
			if (!gameState) return null;
			return getCurrentMove(gameState.game);
		},

		getCurrentTurn: (state) => (boardId: number) => {
			const gameState = state.activeGameMap.get(boardId);
			if (!gameState) return null;
			const position = getCurrentPosition(gameState.game);
			return position ? getTurnFromFen(position.fen) : null;
		},

		getValidMoves: (state) => (boardId: number) => {
			const gameState = state.activeGameMap.get(boardId);
			if (!gameState) return null;
			const position = getCurrentPosition(gameState.game);
			// Note: This getter returns a promise, consider using a computed property with async data
			return position ? getValidMoves(position.fen) : Promise.resolve(null);
		},
	},

	actions: {
		/**
		 * Refresh the game state from the backend session
		 */
		async refreshGameState(boardId: number): Promise<void> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return;

			gameState.isLoading = true;
			gameState.error = null;

			const result = await refreshGameState(boardId);
			if (result.success && result.data) {
				gameState.game = result.data;
			} else {
				gameState.error = result.error || "Failed to refresh game state";
			}

			gameState.isLoading = false;
		},

		/**
		 * Creates a new game session on the specified board
		 */
		async newGame(
			boardId: number,
			type: "standard" | "puzzle" | "960" = "standard",
		): Promise<ActiveGameState | null> {
			// Close existing game if any
			if (this.activeGameMap.has(boardId)) {
				await this.closeGame(boardId);
			}

			const result = await createGameSession(boardId, type);
			if (result.success && result.data) {
				const newGameState: ActiveGameState = {
					id: result.data.id,
					game: result.data.game,
					hideEvaluationBar: false,
					hideBestMove: false,
					hideThreats: false,
					isLoading: false,
					error: null,
				};

				this.activeGameMap.set(boardId, newGameState);
				return newGameState;
			}

			// Set error state if board exists
			const gameState = this.activeGameMap.get(boardId);
			if (gameState) {
				gameState.error = result.error || "Failed to create new game";
				gameState.isLoading = false;
			}

			return null;
		},

		/**
		 * Opens an existing game in a session
		 */
		async openGame(
			gameId: number,
			boardId: number,
		): Promise<ActiveGameState | null> {
			// Close existing game if any
			if (this.activeGameMap.has(boardId)) {
				await this.closeGame(boardId);
			}

			const result = await loadGameSession(gameId, boardId);
			if (result.success && result.data) {
				const newGameState: ActiveGameState = {
					id: result.data.id,
					game: result.data.game,
					hideEvaluationBar: false,
					hideBestMove: false,
					hideThreats: false,
					isLoading: false,
					error: null,
				};

				this.activeGameMap.set(boardId, newGameState);
				return newGameState;
			}

			// Set error state if board exists
			const gameState = this.activeGameMap.get(boardId);
			if (gameState) {
				gameState.error = result.error || "Failed to open game";
				gameState.isLoading = false;
			}

			return null;
		},

		/**
		 * Closes a game session
		 */
		async closeGame(boardId: number): Promise<void> {
			await closeGameSession(boardId);
			// Always remove from local state regardless of backend result
			this.activeGameMap.delete(boardId);
		},

		/**
		 * Makes a move in the game session
		 */
		async makeMove(boardId: number, moveNotation: string): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			gameState.isLoading = true;
			gameState.error = null;

			const result = await makeMove(boardId, moveNotation, gameState.game);
			if (result.success && result.data) {
				gameState.game = result.data;
				gameState.isLoading = false;
				return true;
			}

			gameState.error = result.error || "Failed to make move";
			gameState.isLoading = false;
			return false;
		},

		/**
		 * Undoes the last move (goes to previous position)
		 */
		async previousMove(boardId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			gameState.isLoading = true;
			gameState.error = null;

			const result = await navigateToPreviousMove(boardId);
			if (result.success && result.data) {
				gameState.game = result.data;
				gameState.isLoading = false;
				return true;
			}

			gameState.error = result.error || "Failed to go to previous move";
			gameState.isLoading = false;
			return false;
		},

		/**
		 * Redoes a move (goes to next position)
		 */
		async nextMove(boardId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			// Check if the current node has multiple children
			if (hasMultipleVariations(gameState.game)) {
				// create an alert to the user
				const { handleGeneralError } = useError();
				handleGeneralError(
					"UNEXPECTED",
					"Multiple children found, but the UI does not support picking a variation",
					{
						metadata: { boardId },
					},
				);
			}

			gameState.isLoading = true;
			gameState.error = null;

			const result = await navigateToNextMove(boardId);
			if (result.success && result.data) {
				gameState.game = result.data;
				gameState.isLoading = false;
				return true;
			}

			const { handleAPIError } = useError();
			handleAPIError(result.error, "go to next move", { boardId });
			gameState.isLoading = false;
			gameState.error = result.error || "Failed to go to next move";

			return false;
		},

		/**
		 * Jumps to a specific move number
		 */
		async jumpToMove(boardId: number, moveId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			// TODO: Update api to return partial game state updates
			// so we don't need to fetch the entire game state
			const result = await jumpToMove(boardId, moveId);
			if (result.success && result.data) {
				gameState.game = result.data;
				gameState.isLoading = false;
				return true;
			}

			const { handleAPIError } = useError();
			handleAPIError(result.error, "jump to move", { boardId, moveId });
			gameState.isLoading = false;
			gameState.error = result.error || "Failed to jump to move";
			return false;
		},

		/**
		 * Navigate to the start of the game
		 */
		async navigateToStart(boardId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			const result = await navigateToStart(boardId);
			if (result.success && result.data) {
				gameState.game = result.data;
				gameState.isLoading = false;
				return true;
			}

			const { handleAPIError } = useError();
			handleAPIError(result.error, "navigate to start", { boardId });
			gameState.error = result.error || "Failed to navigate to start";
			gameState.isLoading = false;

			return false;
		},

		/**
		 * Navigate to the end of the main line
		 */
		async navigateToEnd(boardId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			const result = await navigateToEnd(boardId);
			if (result.success && result.data) {
				gameState.game = result.data;
				gameState.isLoading = false;
				return true;
			}

			const { handleAPIError } = useError();
			handleAPIError(result.error, "navigate to end", { boardId });
			gameState.error = result.error || "Failed to navigate to end";
			gameState.isLoading = false;
			return false;
		},

		/**
		 * Saves the current game session to the database
		 */
		async saveGame(boardId: number, overwrite = false): Promise<number | null> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return null;

			gameState.isLoading = true;
			gameState.error = null;

			const result = await saveGameSession(boardId, overwrite);
			if (result.success && result.data) {
				// Update the game ID if it was a new save
				if (!overwrite) {
					gameState.id = result.data;
				}
				gameState.isLoading = false;
				return result.data;
			}

			gameState.error = result.error || "Failed to save game";
			gameState.isLoading = false;
			return null;
		},

		/**
		 * Deletes a game from the database
		 */
		async deleteGame(gameId: number): Promise<boolean> {
			const result = await deleteGameService(gameId);
			if (result.success) {
				// Remove any active sessions using this game
				for (const [boardId, gameState] of this.activeGameMap.entries()) {
					if (gameState.id === gameId) {
						this.activeGameMap.delete(boardId);
					}
				}
				return true;
			}
			return false;
		},

		/**
		 * Updates UI preferences for a game
		 */
		updateUIPreferences(
			boardId: number,
			preferences: Partial<
				Pick<
					ActiveGameState,
					"hideEvaluationBar" | "hideBestMove" | "hideThreats"
				>
			>,
		): void {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return;

			Object.assign(gameState, preferences);
		},

		/**
		 * Clears any error state for a game
		 */
		clearError(boardId: number): void {
			const gameState = this.activeGameMap.get(boardId);
			if (gameState) {
				gameState.error = null;
			}
		},
	},
});
