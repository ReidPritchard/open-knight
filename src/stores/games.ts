import { defineStore } from "pinia";
import api from "../shared/api";
import type {
	ChessGame,
	ChessPosition,
	ChessTreeNode,
	LegalMove,
} from "../shared/bindings";
import { useUIStore } from "./ui";

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

// Helper functions
const getValidMoves = async (
	position?: string,
): Promise<LegalMove[] | null> => {
	if (!position) return null;
	try {
		return await api.moves.GET.validMoves(position);
	} catch (error) {
		console.error("Failed to fetch valid moves:", error);
		return null;
	}
};

const getTurnFromFen = (fen: string): "white" | "black" | null => {
	const turn = fen.split(" ")[1];
	if (!turn) return null;
	return turn === "w" ? "white" : "black";
};

const getCurrentPosition = (game: ChessGame): ChessPosition | null => {
	const currentNodeId = game.move_tree.current_node_id?.idx ?? 0;
	const currentNode = game.move_tree.nodes[currentNodeId];
	return currentNode?.value?.position || null;
};

const getCurrentMove = (game: ChessGame): ChessTreeNode | null => {
	const currentNodeId = game.move_tree.current_node_id?.idx ?? 0;
	const currentNode = game.move_tree.nodes[currentNodeId];
	return currentNode?.value || null;
};

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

			try {
				gameState.isLoading = true;
				gameState.error = null;

				const updatedGame = await api.sessions.GET.get(boardId);
				gameState.game = updatedGame;

				console.log("Refreshed game state:", updatedGame);
			} catch (error) {
				console.error("Failed to refresh game state:", error);
				gameState.error =
					error instanceof Error
						? error.message
						: "Failed to refresh game state";
			} finally {
				gameState.isLoading = false;
			}
		},

		/**
		 * Creates a new game session on the specified board
		 */
		async newGame(
			boardId: number,
			type: "standard" | "puzzle" | "960" = "standard",
		): Promise<ActiveGameState | null> {
			try {
				// Close existing game if any
				if (this.activeGameMap.has(boardId)) {
					await this.closeGame(boardId);
				}

				// Create new session
				const game = await api.sessions.POST.create(boardId, type);
				console.log("New game session:", game);
				console.log("Game Move Tree:", game.move_tree);

				const newGameState: ActiveGameState = {
					id: game.id,
					game: game,
					hideEvaluationBar: false,
					hideBestMove: false,
					hideThreats: false,
					isLoading: false,
					error: null,
				};

				this.activeGameMap.set(boardId, newGameState);
				return newGameState;
			} catch (error) {
				console.error("Failed to create new game:", error);

				// Set error state if board exists
				const gameState = this.activeGameMap.get(boardId);
				if (gameState) {
					gameState.error =
						error instanceof Error
							? error.message
							: "Failed to create new game";
					gameState.isLoading = false;
				}

				return null;
			}
		},

		/**
		 * Opens an existing game in a session
		 */
		async openGame(
			gameId: number,
			boardId: number,
		): Promise<ActiveGameState | null> {
			try {
				// Close existing game if any
				if (this.activeGameMap.has(boardId)) {
					await this.closeGame(boardId);
				}

				// Load game into session
				const game = await api.sessions.POST.load(gameId, boardId);
				console.log("Opened game session:", game);
				console.log("Game Move Tree:", game.move_tree);

				const newGameState: ActiveGameState = {
					id: gameId,
					game: game,
					hideEvaluationBar: false,
					hideBestMove: false,
					hideThreats: false,
					isLoading: false,
					error: null,
				};

				this.activeGameMap.set(boardId, newGameState);
				return newGameState;
			} catch (error) {
				console.error("Failed to open game:", error);

				// Set error state if board exists
				const gameState = this.activeGameMap.get(boardId);
				if (gameState) {
					gameState.error =
						error instanceof Error ? error.message : "Failed to open game";
					gameState.isLoading = false;
				}

				return null;
			}
		},

		/**
		 * Closes a game session
		 */
		async closeGame(boardId: number): Promise<void> {
			try {
				await api.sessions.POST.close(boardId);
				this.activeGameMap.delete(boardId);
				console.log("Closed game session:", boardId);
			} catch (error) {
				console.error("Failed to close game session:", error);
				// Still remove from local state even if backend call fails
				this.activeGameMap.delete(boardId);
			}
		},

		/**
		 * Makes a move in the game session
		 */
		async makeMove(boardId: number, moveNotation: string): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			// Check if the move is new or already in the move tree
			// as a 'next move'. If it already exists, we can just go to the next move.
			// If it's new, add it and set the game as 'dirty'
			const moveTree = gameState.game.move_tree;
			const currentNode = moveTree.nodes[moveTree.current_node_id?.idx ?? 0];
			const childrenNodes = currentNode?.value?.children_ids.map(
				(childId) => moveTree.nodes[childId.idx],
			);
			const childNode = childrenNodes?.find((child) => {
				return child.value?.game_move?.uci === moveNotation;
			});
			if (childNode) {
				console.log("Move already exists, going to next move");
				return this.nextMove(boardId);
			}

			console.log("Move is new, adding to move tree");

			try {
				gameState.isLoading = true;
				gameState.error = null;

				console.log("Making move:", moveNotation);

				const updatedGame = await api.sessions.POST.makeMove(
					boardId,
					moveNotation,
				);
				gameState.game = updatedGame;

				console.log("Move made successfully:", updatedGame);
				return true;
			} catch (error) {
				console.error("Failed to make move:", error);
				gameState.error =
					error instanceof Error ? error.message : "Failed to make move";
				return false;
			} finally {
				gameState.isLoading = false;
			}
		},

		/**
		 * Undoes the last move (goes to previous position)
		 */
		async previousMove(boardId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			try {
				gameState.isLoading = true;
				gameState.error = null;

				const updatedGame = await api.sessions.POST.previousMove(boardId);
				gameState.game = updatedGame;

				console.log("Moved to previous position:", updatedGame);
				return true;
			} catch (error) {
				console.error("Failed to go to previous move:", error);
				gameState.error =
					error instanceof Error
						? error.message
						: "Failed to go to previous move";
				return false;
			} finally {
				gameState.isLoading = false;
			}
		},

		/**
		 * Redoes a move (goes to next position)
		 */
		async nextMove(boardId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			// Check if the current node has multiple children
			const moveTree = gameState.game.move_tree;
			const currentNode = moveTree.nodes[moveTree.current_node_id?.idx ?? 0];

			if (
				currentNode?.value?.children_ids?.length &&
				currentNode.value.children_ids.length > 1
			) {
				console.warn(
					"Multiple children found, but the UI does not support picking a variation",
					currentNode.value.children_ids,
				);
				// create an alert to the user
				useUIStore().addAlert({
					key: "missing-variation-selection-ui",
					message: "Variation selection UI is not yet implemented",
					title: "Variations found",
					type: "error",
				});
			}

			try {
				gameState.isLoading = true;
				gameState.error = null;

				// TODO: Handle variations
				// probably should be a UI component that prompts the user to select a variation
				// or a dropdown menu that shows the variations
				const updatedGame = await api.sessions.POST.nextMove(boardId);
				gameState.game = updatedGame;

				console.log("Moved to next position:", updatedGame);
				return true;
			} catch (error) {
				console.error("Failed to go to next move:", error);
				gameState.error =
					error instanceof Error ? error.message : "Failed to go to next move";
				return false;
			} finally {
				gameState.isLoading = false;
			}
		},

		/**
		 * Jumps to a specific move number
		 */
		async jumpToMove(boardId: number, moveId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			try {
				gameState.isLoading = true;
				gameState.error = null;

				// TODO: Update api to return partial game state updates
				// so we don't need to fetch the entire game state

				await api.sessions.POST.jumpToMove(boardId, moveId);

				// get the updated game state
				const updatedGame = await api.sessions.GET.get(boardId);
				gameState.game = updatedGame;

				console.log("Jumped to move:", moveId, updatedGame);
				return true;
			} catch (error) {
				console.error("Failed to jump to move:", error);
				gameState.error =
					error instanceof Error ? error.message : "Failed to jump to move";
				return false;
			} finally {
				gameState.isLoading = false;
			}
		},

		/**
		 * Navigate to the start of the game
		 */
		async navigateToStart(boardId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			// TODO: Update api to return partial game state updates
			await api.sessions.POST.navigateToStart(boardId);

			// get the updated game state
			const updatedGame = await api.sessions.GET.get(boardId);
			gameState.game = updatedGame;

			console.log("Navigated to start:", updatedGame);
			return true;
		},

		/**
		 * Navigate to the end of the main line
		 */
		async navigateToEnd(boardId: number): Promise<boolean> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return false;

			// Find the last move in the main line
			const moveTree = gameState.game.move_tree;
			if (!moveTree.nodes || moveTree.nodes.length === 0) return false;

			// TODO: Update api to return partial game state updates
			await api.sessions.POST.navigateToEnd(boardId);

			// get the updated game state
			const updatedGame = await api.sessions.GET.get(boardId);
			gameState.game = updatedGame;

			console.log("Navigated to end:", updatedGame);
			return true;
		},

		/**
		 * Saves the current game session to the database
		 */
		async saveGame(boardId: number, overwrite = false): Promise<number | null> {
			const gameState = this.activeGameMap.get(boardId);
			if (!gameState) return null;

			try {
				gameState.isLoading = true;
				gameState.error = null;

				const savedGameId = await api.sessions.POST.save(boardId, overwrite);

				// Update the game ID if it was a new save
				if (!overwrite) {
					gameState.id = savedGameId;
				}

				console.log("Game saved with ID:", savedGameId);
				return savedGameId;
			} catch (error) {
				console.error("Failed to save game:", error);
				gameState.error =
					error instanceof Error ? error.message : "Failed to save game";
				return null;
			} finally {
				gameState.isLoading = false;
			}
		},

		/**
		 * Deletes a game from the database
		 */
		async deleteGame(gameId: number): Promise<boolean> {
			try {
				await api.games.POST.delete(gameId);
				this.activeGameMap.delete(gameId);
				return true;
			} catch (error) {
				console.error("Failed to delete game:", error);
				return false;
			}
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
