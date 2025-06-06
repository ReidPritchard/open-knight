import api from "../shared/api";
import type {
	ChessGame,
	ChessPosition,
	ChessTreeNode,
	LegalMove,
} from "../shared/bindings";
import { ErrorFactory, ErrorHandler } from "./ErrorService";

/**
 * Result of a game operation
 */
export interface GameOperationResult<T = void> {
	success: boolean;
	data?: T;
	error?: string;
}

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
	try {
		return await api.moves.GET.validMoves(position);
	} catch (error) {
		ErrorHandler.handle(
			ErrorFactory.chessGame(
				"INVALID_POSITION",
				`Failed to get valid moves for position: ${position}`,
				{
					metadata: { position },
				},
			),
		);
		return null;
	}
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
): Promise<GameOperationResult<GameSession>> {
	try {
		const game = await api.sessions.POST.create(boardId, type);
		console.log("New game session:", game);

		return {
			success: true,
			data: { id: game.id, game },
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to create new game";
		ErrorHandler.handle(
			ErrorFactory.chessGame(
				"INVALID_POSITION",
				`Failed to create game session: ${errorMessage}`,
				{
					metadata: { boardId, type },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Load an existing game into a session
 */
export async function loadGameSession(
	gameId: number,
	boardId: number,
): Promise<GameOperationResult<GameSession>> {
	try {
		const game = await api.sessions.POST.load(gameId, boardId);
		console.log("Loaded game session:", game);

		return {
			success: true,
			data: { id: gameId, game },
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to load game";
		ErrorHandler.handle(
			ErrorFactory.chessGame(
				"INVALID_POSITION",
				`Failed to load game session: ${errorMessage}`,
				{
					metadata: { gameId, boardId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Close a game session
 */
export async function closeGameSession(
	boardId: number,
): Promise<GameOperationResult> {
	try {
		await api.sessions.POST.close(boardId);
		console.log("Closed game session:", boardId);
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to close session";
		ErrorHandler.handle(
			ErrorFactory.general(
				"UNEXPECTED",
				`Failed to close game session: ${errorMessage}`,
				{
					metadata: { boardId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Make a move in the game
 */
export async function makeMove(
	boardId: number,
	moveNotation: string,
	currentGame: ChessGame,
): Promise<GameOperationResult<ChessGame>> {
	try {
		// Check if move already exists in the tree
		if (moveExistsInTree(currentGame, moveNotation)) {
			console.log("Move already exists, navigating to next move");
			return await navigateToNextMove(boardId);
		}

		console.log("Move is new, adding to move tree");
		const updatedGame = await api.sessions.POST.makeMove(boardId, moveNotation);
		console.log("Move made successfully:", updatedGame);

		return {
			success: true,
			data: updatedGame,
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to make move";
		ErrorHandler.handle(
			ErrorFactory.chessGame(
				"INVALID_MOVE",
				`Failed to make move: ${errorMessage}`,
				{
					metadata: { boardId, moveNotation },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Navigate to the previous move
 */
export async function navigateToPreviousMove(
	boardId: number,
): Promise<GameOperationResult<ChessGame>> {
	try {
		const updatedGame = await api.sessions.POST.previousMove(boardId);
		console.log("Moved to previous position:", updatedGame);
		return {
			success: true,
			data: updatedGame,
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to go to previous move";
		ErrorHandler.handle(
			ErrorFactory.chessGame(
				"INVALID_POSITION",
				`Failed to navigate to previous move: ${errorMessage}`,
				{
					metadata: { boardId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Navigate to the next move
 */
export async function navigateToNextMove(
	boardId: number,
): Promise<GameOperationResult<ChessGame>> {
	try {
		const updatedGame = await api.sessions.POST.nextMove(boardId);
		console.log("Moved to next position:", updatedGame);
		return {
			success: true,
			data: updatedGame,
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to go to next move";
		ErrorHandler.handle(
			ErrorFactory.chessGame(
				"INVALID_POSITION",
				`Failed to navigate to next move: ${errorMessage}`,
				{
					metadata: { boardId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Jump to a specific move
 */
export async function jumpToMove(
	boardId: number,
	moveId: number,
): Promise<GameOperationResult<ChessGame>> {
	try {
		await api.sessions.POST.jumpToMove(boardId, moveId);
		const updatedGame = await api.sessions.GET.get(boardId);
		return {
			success: true,
			data: updatedGame,
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to jump to move";
		ErrorHandler.handle(
			ErrorFactory.chessGame(
				"INVALID_POSITION",
				`Failed to jump to move: ${errorMessage}`,
				{
					metadata: { boardId, moveId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Navigate to the start of the game
 */
export async function navigateToStart(
	boardId: number,
): Promise<GameOperationResult<ChessGame>> {
	try {
		await api.sessions.POST.navigateToStart(boardId);
		const updatedGame = await api.sessions.GET.get(boardId);
		console.log("Navigated to start:", updatedGame);
		return {
			success: true,
			data: updatedGame,
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to navigate to start";
		ErrorHandler.handle(
			ErrorFactory.chessGame(
				"INVALID_POSITION",
				`Failed to navigate to start: ${errorMessage}`,
				{
					metadata: { boardId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Navigate to the end of the main line
 */
export async function navigateToEnd(
	boardId: number,
): Promise<GameOperationResult<ChessGame>> {
	try {
		await api.sessions.POST.navigateToEnd(boardId);
		const updatedGame = await api.sessions.GET.get(boardId);
		console.log("Navigated to end:", updatedGame);
		return {
			success: true,
			data: updatedGame,
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to navigate to end";
		ErrorHandler.handle(
			ErrorFactory.chessGame(
				"INVALID_POSITION",
				`Failed to navigate to end: ${errorMessage}`,
				{
					metadata: { boardId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Save the current game session
 */
export async function saveGameSession(
	boardId: number,
	overwrite = false,
): Promise<GameOperationResult<number>> {
	try {
		const savedGameId = await api.sessions.POST.save(boardId, overwrite);
		console.log("Game saved with ID:", savedGameId);
		return {
			success: true,
			data: savedGameId,
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to save game";
		ErrorHandler.handle(
			ErrorFactory.database(
				"INSERT_ERROR",
				`Failed to save game: ${errorMessage}`,
				{
					metadata: { boardId, overwrite },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Delete a game from the database
 */
export async function deleteGame(gameId: number): Promise<GameOperationResult> {
	try {
		await api.games.POST.delete(gameId);
		return { success: true };
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to delete game";
		ErrorHandler.handle(
			ErrorFactory.database(
				"QUERY_ERROR",
				`Failed to delete game: ${errorMessage}`,
				{
					metadata: { gameId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}

/**
 * Refresh game state from backend
 */
export async function refreshGameState(
	boardId: number,
): Promise<GameOperationResult<ChessGame>> {
	try {
		const updatedGame = await api.sessions.GET.get(boardId);
		console.log("Refreshed game state:", updatedGame);
		return {
			success: true,
			data: updatedGame,
		};
	} catch (error) {
		const errorMessage =
			error instanceof Error ? error.message : "Failed to refresh game state";
		ErrorHandler.handle(
			ErrorFactory.general(
				"UNEXPECTED",
				`Failed to refresh game state: ${errorMessage}`,
				{
					metadata: { boardId },
				},
			),
		);
		return {
			success: false,
			error: errorMessage,
		};
	}
}
