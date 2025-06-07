import { API } from "../shared/api";
import type {
	ChessGame,
	ChessPosition,
	ChessTreeNode,
	LegalMove,
} from "../shared/bindings";
import type { OperationResult } from "../shared/types";
import { ErrorCategory, withErrorHandling } from "./ErrorService";

/**
 * Game session data
 */
export interface GameSession {
	id: number;
	game: ChessGame;
}

/**
 * Extract the current position from a chess game
 */
export function getCurrentPosition(game: ChessGame): ChessPosition | null {
	const currentNodeId = game.move_tree.current_node_id?.idx ?? 0;
	const currentNode = game.move_tree.nodes[currentNodeId];
	return currentNode?.value?.position || null;
}

/**
 * Extract the current move from a chess game
 */
export function getCurrentMove(game: ChessGame): ChessTreeNode | null {
	const currentNodeId = game.move_tree.current_node_id?.idx ?? 0;
	const currentNode = game.move_tree.nodes[currentNodeId];
	return currentNode?.value || null;
}

/**
 * Get the current turn from a FEN string
 */
export function getTurnFromFen(fen: string): "white" | "black" | null {
	const turn = fen.split(" ")[1];
	if (!turn) return null;
	return turn === "w" ? "white" : "black";
}

/**
 * Get valid moves for a position
 */
export async function getValidMoves(
	position?: string,
): Promise<LegalMove[] | null> {
	if (!position) return null;

	const result = await withErrorHandling(
		() => API.analysis.getValidMoves(position),
		ErrorCategory.CHESS_GAME,
		"INVALID_POSITION",
		`Failed to get valid moves for position: ${position}`,
		{ position },
	);

	return result.success ? (result.data ?? null) : null;
}

/**
 * Check if current node has multiple children (variations)
 */
export function hasMultipleVariations(game: ChessGame): boolean {
	const moveTree = game.move_tree;
	const currentNode = moveTree.nodes[moveTree.current_node_id?.idx ?? 0];
	return (currentNode?.value?.children_ids?.length ?? 0) > 1;
}

/**
 * Check if move already exists in the move tree
 */
export function moveExistsInTree(
	game: ChessGame,
	moveNotation: string,
): boolean {
	const moveTree = game.move_tree;
	const currentNode = moveTree.nodes[moveTree.current_node_id?.idx ?? 0];
	const childrenNodes = currentNode?.value?.children_ids.map(
		(childId) => moveTree.nodes[childId.idx],
	);
	return (
		childrenNodes?.some(
			(child) => child.value?.game_move?.uci === moveNotation,
		) ?? false
	);
}

/**
 * Create a new game session
 */
export async function createGameSession(
	boardId: number,
	type: "standard" | "puzzle" | "960" = "standard",
): Promise<OperationResult<GameSession>> {
	const result = await withErrorHandling(
		async () => {
			const game = await API.games.create(boardId, type);
			console.log("New game session:", game);
			return { id: game.id, game };
		},
		ErrorCategory.CHESS_GAME,
		"INVALID_POSITION",
		"Failed to create new game",
		{ boardId, type },
	);

	return result;
}

/**
 * Load an existing game into a session
 */
export async function loadGameSession(
	gameId: number,
	boardId: number,
): Promise<OperationResult<GameSession>> {
	const result = await withErrorHandling(
		async () => {
			const game = await API.board.open(gameId, boardId);
			console.log("Loaded game session:", game);
			return { id: gameId, game };
		},
		ErrorCategory.CHESS_GAME,
		"INVALID_POSITION",
		"Failed to load game",
		{ gameId, boardId },
	);

	return result;
}

/**
 * Close a game session
 */
export async function closeGameSession(
	boardId: number,
): Promise<OperationResult> {
	const result = await withErrorHandling(
		async () => {
			await API.board.close(boardId);
			console.log("Closed game session:", boardId);
		},
		ErrorCategory.GENERAL,
		"UNEXPECTED",
		"Failed to close session",
		{ boardId },
	);

	return result;
}

/**
 * Make a move in the game
 */
export async function makeMove(
	boardId: number,
	moveNotation: string,
	currentGame: ChessGame,
): Promise<OperationResult<ChessGame>> {
	const result = await withErrorHandling(
		async () => {
			// Check if move already exists in the tree
			if (moveExistsInTree(currentGame, moveNotation)) {
				console.log("Move already exists, navigating to next move");
				const nextMoveResult = await navigateToNextMove(boardId);
				if (!nextMoveResult.success || !nextMoveResult.data) {
					throw new Error(
						nextMoveResult.error || "Failed to navigate to next move",
					);
				}
				return nextMoveResult.data;
			}

			console.log("Move is new, adding to move tree");
			const updatedGame = await API.board.move(boardId, moveNotation);
			console.log("Move made successfully:", updatedGame);
			return updatedGame;
		},
		ErrorCategory.CHESS_GAME,
		"INVALID_MOVE",
		"Failed to make move",
		{ boardId, moveNotation },
	);

	return result;
}

/**
 * Navigate to the previous move
 */
export async function navigateToPreviousMove(
	boardId: number,
): Promise<OperationResult<ChessGame>> {
	const result = await withErrorHandling(
		async () => {
			const updatedGame = await API.board.previous(boardId);
			console.log("Moved to previous position:", updatedGame);
			return updatedGame;
		},
		ErrorCategory.CHESS_GAME,
		"INVALID_POSITION",
		"Failed to go to previous move",
		{ boardId },
	);

	return result;
}

/**
 * Navigate to the next move
 */
export async function navigateToNextMove(
	boardId: number,
): Promise<OperationResult<ChessGame>> {
	const result = await withErrorHandling(
		async () => {
			const updatedGame = await API.board.next(boardId);
			console.log("Moved to next position:", updatedGame);
			return updatedGame;
		},
		ErrorCategory.CHESS_GAME,
		"INVALID_POSITION",
		"Failed to go to next move",
		{ boardId },
	);

	return result;
}

/**
 * Jump to a specific move
 */
export async function jumpToMove(
	boardId: number,
	moveId: number,
): Promise<OperationResult<ChessGame>> {
	const result = await withErrorHandling(
		async () => {
			await API.board.jumpTo(boardId, moveId);
			const updatedGame = await API.board.getState(boardId);
			return updatedGame;
		},
		ErrorCategory.CHESS_GAME,
		"INVALID_POSITION",
		"Failed to jump to move",
		{ boardId, moveId },
	);

	return result;
}

/**
 * Navigate to the start of the game
 */
export async function navigateToStart(
	boardId: number,
): Promise<OperationResult<ChessGame>> {
	const result = await withErrorHandling(
		async () => {
			await API.board.toStart(boardId);
			const updatedGame = await API.board.getState(boardId);
			console.log("Navigated to start:", updatedGame);
			return updatedGame;
		},
		ErrorCategory.CHESS_GAME,
		"INVALID_POSITION",
		"Failed to navigate to start",
		{ boardId },
	);

	return result;
}

/**
 * Navigate to the end of the main line
 */
export async function navigateToEnd(
	boardId: number,
): Promise<OperationResult<ChessGame>> {
	return await withErrorHandling(
		() => API.board.toEnd(boardId),
		ErrorCategory.CHESS_GAME,
		"INVALID_POSITION",
		"Failed to navigate to end",
		{ boardId },
	);
}

/**
 * Save the current game session
 */
export async function saveGameSession(
	boardId: number,
	overwrite = false,
): Promise<OperationResult<number>> {
	return await withErrorHandling(
		() => API.board.save(boardId, overwrite),
		ErrorCategory.DATABASE,
		"SAVE_ERROR",
		"Failed to save game",
		{ metadata: { boardId, overwrite } },
	);
}

/**
 * Delete a game from the database
 */
export async function deleteGame(gameId: number): Promise<OperationResult> {
	return await withErrorHandling(
		() => API.games.delete(gameId),
		ErrorCategory.DATABASE,
		"DELETE_ERROR",
		"Failed to delete game",
		{ metadata: { gameId } },
	);
}

/**
 * Refresh game state from backend
 */
export async function refreshGameState(
	boardId: number,
): Promise<OperationResult<ChessGame>> {
	return await withErrorHandling(
		() => API.board.getState(boardId),
		ErrorCategory.DATABASE,
		"REFRESH_ERROR",
		"Failed to refresh game state",
		{ metadata: { boardId } },
	);
}
