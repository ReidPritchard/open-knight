<template>

	<div
		class="relative"
		:style="`width: ${squareSizePixels * 8}px; height: ${
			squareSizePixels * 8
		}px;`"
	>

		<!-- Chess Board Grid -->

		<div
			class="grid grid-rows-8 grid-flow-col transition-all duration-100 w-full h-full"
			:class="{ 'rotate-180': isBoardFlipped }"
		>

			<div
				v-for="row in 8"
				:key="row - 1"
				class="flex"
			>

				<ChessBoardSquare
					v-for="col in 8"
					:key="col - 1"
					:row="row - 1"
					:col="col - 1"
					:square-size="squareSizePixels"
					:piece="getPieceAtCoords(row - 1, col - 1)"
					:piece-image="
						getPieceImagePath(getPieceAtCoords(row - 1, col - 1) ?? '')
					"
					:can-move="canMovePiece(row - 1, col - 1)"
					:is-selected="isSquareSelected(row - 1, col - 1)"
					:is-valid-move="isValidMoveTarget(row - 1, col - 1)"
					:is-highlighted="isPartOfCurrentMove(row - 1, col - 1)"
					:is-board-flipped="isBoardFlipped"
					:board-theme="boardTheme"
					:class="{ 'rotate-180': isBoardFlipped, 'rotate-0': !isBoardFlipped }"
					@drop="handleDrop(row - 1, col - 1)"
					@click="handleSquareClick($event, row - 1, col - 1)"
					@contextmenu.prevent="
						handleSquareContextMenu($event, row - 1, col - 1)
					"
					@drag-start="handleDragStart(row - 1, col - 1)"
				/>

			</div>

		</div>

		<!-- Annotation Arrow -->

		<div
			v-if="arrowCoordinates"
			class="absolute inset-0 pointer-events-none"
			@contextmenu.prevent
		>

			<AnnotationArrow
				:from="arrowCoordinates.from"
				:to="arrowCoordinates.to"
				:options="{ color: 'yellow', size: 5 }"
			/>

		</div>

		<!-- Temporary Annotation Arrows (user-created, not saved) -->

		<div
			v-if="temporaryAnnotations.length > 0"
			v-for="(annotation, index) in temporaryAnnotations"
			class="absolute inset-0 pointer-events-none"
			@contextmenu.prevent
		>

			<AnnotationArrow
				:key="`temp-${index}`"
				:from="annotation.coordinates.from"
				:to="annotation.coordinates.to"
				:options="annotation.options"
			/>

		</div>

		<!-- Active Arrow Being Drawn (real-time feedback during drag) -->

		<div
			v-if="activeArrow"
			class="absolute inset-0"
			@contextmenu.prevent
		>

			<AnnotationArrow
				:from="activeArrow.from"
				:to="activeArrow.to"
				:options="activeArrow.options"
			/>

		</div>

	</div>

	<!-- Move Navigation -->

	<div class="flex flex-row items-center justify-center mt-4">

		<div class="join">

			<button
				class="join-item btn"
				:disabled="!currentMove"
				@click="handlePreviousMove"
			>

				<PhArrowLeft />

			</button>

			<span class="label join-item px-8 w-40"> {{ formatCurrentMove }} </span>

			<button
				class="join-item btn"
				:disabled="!hasNextMove"
				@click="handleNextMove"
			>

				<PhArrowRight />

			</button>

		</div>

		<!-- Rotate Board -->

		<button
			class="btn btn-sm ml-4"
			@click="rotateBoard"
		>

			<PhArrowsClockwise size="16" />

		</button>

		<!-- Resize Board (drag to resize) -->

		<div
			class="ml-4 cursor-ew-resize"
			@mousedown="startResize"
		>

			<PhArrowsOutLineHorizontal size="16" />

		</div>

		<!-- Clear Annotations -->

		<button
			v-if="temporaryAnnotations.length > 0"
			class="btn btn-sm ml-4"
			@click="clearAnnotations"
			title="Clear annotations (Escape)"
		>

			<PhX size="16" />

		</button>

	</div>

</template>

<script setup lang="ts">
/*
 * Chess Board Component with Annotation System
 *
 * Annotation Features:
 * - Right-click and drag to create arrows between squares
 * - Color selection based on modifier keys:
 *   - Default (no modifier): Yellow
 *   - Shift: Green
 *   - Ctrl/Cmd: Red
 *   - Alt: Blue
 * - Multiple arrows supported
 * - Escape key or clear button to remove all annotations
 * - Temporary annotations (not saved to database)
 * - Real-time visual feedback while drawing
 *
 * Usage:
 * - Right-click on a square and drag to another square to create an arrow
 * - Hold modifier keys while right-clicking to change arrow color
 * - Press Escape or click the X button to clear all annotations
 */

import {
	PhArrowLeft,
	PhArrowRight,
	PhArrowsClockwise,
	PhArrowsOutLineHorizontal,
	PhX,
} from "@phosphor-icons/vue";
import { computed, onMounted, onUnmounted, ref, watch } from "vue";
import api from "../../shared/api";
import { useGlobalStore } from "../../stores/";
import { useError } from "../../composables/useError";
import AnnotationArrow from "../AnnotationArrow/AnnotationArrow.vue";
import ChessBoardSquare from "./ChessBoardSquare.vue";
import {
	algebraicToBoard,
	boardToAlgebraic,
	calculateSquareCenter,
	getPieceImagePath,
	isAnnotationClick,
	isValidPiece,
	isWhitePiece,
	parseFen,
	parseUciMove,
} from "./utils";

// ---------------
// Props and emits
// ---------------
const props = defineProps<{
	boardId: number;
}>();

const emit = defineEmits<{
	(e: "move", move: { from: string; to: string }): void;
	(e: "error", error: Error): void;
	(e: "previousMove"): void;
	(e: "nextMove"): void;
}>();

// ---------------
// Store and state
// ---------------
const globalStore = useGlobalStore();
const gamesStore = globalStore.gamesStore;

// Board state from store
const boardState = computed(() => gamesStore.getBoardState(props.boardId));

// Use the new getters from the refactored store
const currentMove = computed(() => gamesStore.getCurrentMove(props.boardId));
const currentPosition = computed(() =>
	gamesStore.getCurrentPosition(props.boardId),
);
const currentTurn = computed(() => gamesStore.getCurrentTurn(props.boardId));

// For valid moves, we need to handle the async nature
const validMoves = ref<Array<{ uci: string; san: string }> | null>(null);

// Watch for position changes and update valid moves
watch(
	currentPosition,
	async (newPosition) => {
		if (newPosition?.fen) {
			try {
				validMoves.value = await api.moves.GET.validMoves(newPosition.fen);
			} catch (error) {
				const { handleAPIError } = useError();
				handleAPIError(error, "fetch valid moves", { fen: newPosition.fen });
				validMoves.value = null;
			}
		} else {
			validMoves.value = null;
		}
	},
	{ immediate: true },
);

// Board orientation
const isBoardFlipped = computed(
	() => globalStore.uiStore.whiteOnSide === "top",
);

// Board styling
const squareSizePixels = computed(() => globalStore.uiStore.boardSquareSize);
const boardTheme = computed(() => globalStore.uiStore.boardTheme);

// Move navigation - derive from game tree
const hasNextMove = computed(() => {
	if (!boardState.value?.game?.move_tree) return false;

	const currentNodeId =
		boardState.value.game.move_tree.current_node_id?.idx ?? 0;
	const currentNode = boardState.value.game.move_tree.nodes[currentNodeId];

	return (currentNode?.value?.children_ids?.length ?? 0) > 0;
});

const formatCurrentMove = computed(() => {
	const move = currentMove.value;
	if (!move?.game_move?.ply_number || !move?.game_move?.san) return "Start";

	const moveNumber = Math.floor(move.game_move.ply_number / 2) + 1;
	const isWhiteMove = move.game_move.ply_number % 2 === 1;

	if (isWhiteMove) {
		return `${moveNumber}. ${move.game_move.san}`;
	}

	return `${moveNumber}... ${move.game_move.san}`;
});

// Selected piece state
const selectedPiece = ref<{ row: number; col: number } | null>(null);
const validPieceMoves = ref<Array<{ row: number; col: number }>>([]);

// Annotation state
interface TemporaryAnnotation {
	coordinates: {
		from: { x: number; y: number };
		to: { x: number; y: number };
	};
	options: {
		color: string;
		size: number;
	};
	squares: {
		from: string;
		to: string;
	};
}

interface ActiveArrow {
	from: { x: number; y: number };
	to: { x: number; y: number };
	options: {
		color: string;
		size: number;
	};
}

const temporaryAnnotations = ref<TemporaryAnnotation[]>([]);
const activeArrow = ref<ActiveArrow | null>(null);
const isDrawingArrow = ref(false);
const arrowStartSquare = ref<{ row: number; col: number } | null>(null);

// Global mouse event handlers for annotation drawing
let currentMouseMoveHandler: ((event: MouseEvent) => void) | null = null;
let currentMouseUpHandler: ((event: MouseEvent) => void) | null = null;

// Cleanup function for annotation drawing
const cleanupAnnotationDrawing = () => {
	console.log("Cleaning up annotation drawing state");
	isDrawingArrow.value = false;
	arrowStartSquare.value = null;
	activeArrow.value = null;

	if (currentMouseMoveHandler) {
		document.removeEventListener("mousemove", currentMouseMoveHandler);
		currentMouseMoveHandler = null;
	}
	if (currentMouseUpHandler) {
		document.removeEventListener("mouseup", currentMouseUpHandler);
		currentMouseUpHandler = null;
	}
	console.log("Cleanup completed");
};

// Annotation colors based on modifier keys
const getAnnotationColor = (event: MouseEvent): string => {
	if (event.shiftKey) return "#22c55e"; // green
	if (event.ctrlKey || event.metaKey) return "#ef4444"; // red
	if (event.altKey) return "#3b82f6"; // blue
	return "#eab308"; // yellow (default)
};

// Valid moves computed from store - single source of truth
const validMovesMap = computed(() => {
	const moves = validMoves.value || [];
	const movesMap: Record<string, Array<{ row: number; col: number }>> = {};

	for (const move of moves) {
		const { from, to } = parseUciMove(move.uci);
		const { row: fromRow, col: fromCol } = algebraicToBoard(from);
		const { row: toRow, col: toCol } = algebraicToBoard(to);

		const fromKey = `${fromRow},${fromCol}`;
		if (!movesMap[fromKey]) {
			movesMap[fromKey] = [];
		}
		movesMap[fromKey].push({ row: toRow, col: toCol });
	}

	return movesMap;
});

// ---------------
// Coordinate transformation utilities
// ---------------

/**
 * Gets a piece at the specified coordinates
 * @param row 0-based row index (0-7, 0 = top)
 * @param col 0-based column index (0-7, 0 = left)
 * @returns The piece at the specified position, or undefined if none
 */
function getPieceAtCoords(row: number, col: number) {
	const board = currentPosition.value?.fen
		? parseFen(currentPosition.value.fen)
		: undefined;
	if (!board) return undefined;

	// Access the board array directly with coordinates
	const piece = board[row]?.[col];
	return isValidPiece(piece) ? piece : undefined;
}

/**
 * Checks if a piece at the specified coordinates can be moved
 * @param row 0-based row index (0-7, 0 = top)
 * @param col 0-based column index (0-7, 0 = left)
 * @returns True if the piece can be moved, false otherwise
 */
function canMovePiece(row: number, col: number) {
	// Check if it's the player's turn
	const piece = getPieceAtCoords(row, col);
	if (!piece) return false;

	const isWhite = isWhitePiece(piece);
	const isWhiteTurn = currentTurn.value === "white";

	if (isWhite !== isWhiteTurn) {
		return false;
	}

	// Check if the piece has valid moves
	const positionMoves = validMovesMap.value[`${row},${col}`] || [];
	return positionMoves.length > 0;
}

/**
 * Checks if a square is currently selected
 * @param row 0-based row index (0-7, 0 = top)
 * @param col 0-based column index (0-7, 0 = left)
 * @returns True if the square is selected, false otherwise
 */
function isSquareSelected(row: number, col: number) {
	if (!selectedPiece.value) return false;
	return selectedPiece.value.row === row && selectedPiece.value.col === col;
}

/**
 * Checks if a square is a valid move target for the selected piece
 * @param row 0-based row index (0-7, 0 = top)
 * @param col 0-based column index (0-7, 0 = left)
 * @returns True if the square is a valid move target, false otherwise
 */
function isValidMoveTarget(row: number, col: number) {
	return validPieceMoves.value.some(
		(move) => move.row === row && move.col === col,
	);
}

/**
 * Checks if the current square is part of the current move
 * @param row 0-based row index (0-7, 0 = top)
 * @param col 0-based column index (0-7, 0 = left)
 * @returns True if the square is part of the current move, false otherwise
 */
function isPartOfCurrentMove(row: number, col: number) {
	if (!currentMove?.value?.game_move?.uci) return false;

	const { from, to } = parseUciMove(currentMove.value.game_move.uci);

	// Convert algebraic notation to board coordinates
	const fromCoords = algebraicToBoard(from);
	const toCoords = algebraicToBoard(to);

	// Check if current square is either the from or to square
	return (
		(row === fromCoords.row && col === fromCoords.col) ||
		(row === toCoords.row && col === toCoords.col)
	);
}

// ---------------
// Annotations
// ---------------
interface Annotation {
	comment?: string | null;
	arrows?: string | null;
	highlights?: string | null;
}

const annotations = computed(
	() =>
		currentMove.value?.game_move?.annotations?.map((annotation: Annotation) => {
			const { comment, arrows, highlights } = annotation;
			return {
				comment,
				arrows: arrows ? parseUciMove(arrows) : null,
				highlights: highlights ? algebraicToBoard(highlights) : null,
			};
		}) || [],
);

const arrowCoordinates = computed(() => {
	if (!annotations.value?.length) return null;

	const validArrows = annotations.value.filter(
		(annotation) => annotation.arrows,
	);
	if (!validArrows.length || !validArrows[0].arrows) return null;

	const { arrows } = validArrows[0];

	// Convert algebraic notation to coordinates
	const fromCoords = algebraicToBoard(arrows.from);
	const toCoords = algebraicToBoard(arrows.to);

	// For visual elements like arrows, we need to consider board rotation
	// since the actual pixel positions change when the board is flipped
	return {
		from: calculateSquareCenter(
			fromCoords.col,
			fromCoords.row,
			squareSizePixels.value,
			isBoardFlipped.value,
		),
		to: calculateSquareCenter(
			toCoords.col,
			toCoords.row,
			squareSizePixels.value,
			isBoardFlipped.value,
		),
	};
});

// ---------------
// Move handling
// ---------------
function getValidMoves(row: number, col: number) {
	const piece = getPieceAtCoords(row, col);
	if (!piece) return [];

	// Get moves directly from computed property
	const cacheKey = `${row},${col}`;
	return validMovesMap.value[cacheKey] || [];
}

// ---------------
// Event handlers
// ---------------
function handleDragStart(row: number, col: number) {
	selectedPiece.value = { row, col };
	validPieceMoves.value = getValidMoves(row, col);
}

async function handleDrop(row: number, col: number) {
	if (!selectedPiece.value) return;

	// Check if the move is valid
	if (!isValidMoveTarget(row, col)) return;

	// Convert coordinates to algebraic notation
	const fromSquare = boardToAlgebraic(
		selectedPiece.value.row,
		selectedPiece.value.col,
	);
	const toSquare = boardToAlgebraic(row, col);
	const moveNotation = fromSquare + toSquare;

	try {
		await gamesStore.makeMove(props.boardId, moveNotation);
		gamesStore.nextMove(props.boardId);
		globalStore.uiStore.updateBoardMetadata(props.boardId, {
			hasUnsavedChanges: true,
		});
	} catch (error) {
		const { handleAPIError } = useError();
		handleAPIError(error, "make move", {
			boardId: props.boardId,
			moveNotation,
		});
	} finally {
		// Reset selection regardless of success/failure
		selectedPiece.value = null;
		validPieceMoves.value = [];
	}
}

/**
 * Handles a click on a square
 * @param event The mouse event
 * @param row The row of the square (0-7, 0 = top)
 * @param col The column of the square (0-7, 0 = left)
 */
async function handleSquareClick(_event: MouseEvent, row: number, col: number) {
	const piece = getPieceAtCoords(row, col);

	if (selectedPiece.value && isValidMoveTarget(row, col)) {
		// Move the selected piece to this square
		await handleDrop(row, col);
		return;
	}

	if (piece && canMovePiece(row, col) && !isSquareSelected(row, col)) {
		// Select this piece
		selectedPiece.value = { row, col };
		validPieceMoves.value = getValidMoves(row, col);
		return;
	}

	// Deselect
	selectedPiece.value = null;
	validPieceMoves.value = [];
}

function handleSquareContextMenu(event: MouseEvent, row: number, col: number) {
	console.log("Square context menu", event);
	console.log("Event details:", {
		button: event.button,
		shiftKey: event.shiftKey,
		ctrlKey: event.ctrlKey,
		metaKey: event.metaKey,
		altKey: event.altKey,
	});
	console.log("isAnnotationClick result:", isAnnotationClick(event));
	console.log("Current state:", {
		isDrawingArrow: isDrawingArrow.value,
		hasActiveArrow: !!activeArrow.value,
		hasArrowStartSquare: !!arrowStartSquare.value,
		temporaryAnnotationsCount: temporaryAnnotations.value.length,
	});

	if (isAnnotationClick(event)) {
		console.log("Processing as annotation click");
		event.preventDefault();
		event.stopPropagation();

		// Don't start annotation if a piece is selected (to avoid conflicts)
		if (selectedPiece.value) {
			selectedPiece.value = null;
			validPieceMoves.value = [];
			return;
		}

		// Don't start new annotation if already drawing
		if (isDrawingArrow.value) {
			console.log("Already drawing an arrow, ignoring new attempt");
			return;
		}

		// Start drawing an arrow
		isDrawingArrow.value = true;
		arrowStartSquare.value = { row, col };

		const startCoords = calculateSquareCenter(
			col,
			row,
			squareSizePixels.value,
			isBoardFlipped.value,
		);

		const color = getAnnotationColor(event);

		activeArrow.value = {
			from: startCoords,
			to: startCoords, // Start with same position
			options: {
				color,
				size: 6,
			},
		};

		// Add global mouse event listeners for drag
		const handleMouseMove = (moveEvent: MouseEvent) => {
			if (!isDrawingArrow.value || !activeArrow.value) return;

			// Get mouse position relative to the board
			const boardElement = (moveEvent.target as Element)?.closest(".relative");
			if (!boardElement) return;

			const rect = boardElement.getBoundingClientRect();
			const x = moveEvent.clientX - rect.left;
			const y = moveEvent.clientY - rect.top;

			// Update the active arrow's end position
			activeArrow.value.to = { x, y };
		};

		const handleMouseUp = (upEvent: MouseEvent) => {
			if (
				!isDrawingArrow.value ||
				!arrowStartSquare.value ||
				!activeArrow.value
			) {
				cleanupAnnotationDrawing();
				return;
			}

			// Find which square the mouse ended on
			const boardElement = (upEvent.target as Element)?.closest(".relative");
			if (!boardElement) {
				cleanupAnnotationDrawing();
				return;
			}

			const rect = boardElement.getBoundingClientRect();
			const x = upEvent.clientX - rect.left;
			const y = upEvent.clientY - rect.top;

			// Convert pixel coordinates to square coordinates
			const squareSize = squareSizePixels.value;
			let endCol = Math.floor(x / squareSize);
			let endRow = Math.floor(y / squareSize);

			// Handle board rotation
			if (isBoardFlipped.value) {
				endCol = 7 - endCol;
				endRow = 7 - endRow;
			}

			// Ensure coordinates are within bounds
			if (endCol >= 0 && endCol < 8 && endRow >= 0 && endRow < 8) {
				const startSquare = arrowStartSquare.value;

				// Only create arrow if it's not the same square
				if (startSquare.row !== endRow || startSquare.col !== endCol) {
					const endCoords = calculateSquareCenter(
						endCol,
						endRow,
						squareSizePixels.value,
						isBoardFlipped.value,
					);

					// Create the temporary annotation
					const annotation: TemporaryAnnotation = {
						coordinates: {
							from: activeArrow.value.from,
							to: endCoords,
						},
						options: activeArrow.value.options,
						squares: {
							from: boardToAlgebraic(startSquare.row, startSquare.col),
							to: boardToAlgebraic(endRow, endCol),
						},
					};

					// Check if an arrow already exists between these squares
					const existingIndex = temporaryAnnotations.value.findIndex(
						(ann) =>
							ann.squares.from === annotation.squares.from &&
							ann.squares.to === annotation.squares.to,
					);

					if (existingIndex >= 0) {
						// Replace existing arrow (allows changing color)
						temporaryAnnotations.value[existingIndex] = annotation;
					} else {
						// Add new arrow
						temporaryAnnotations.value.push(annotation);
					}
				}
			}

			cleanupAnnotationDrawing();
		};

		// Add global event listeners
		console.log("Adding global event listeners for annotation drawing");
		currentMouseMoveHandler = handleMouseMove;
		currentMouseUpHandler = handleMouseUp;
		document.addEventListener("mousemove", handleMouseMove);
		document.addEventListener("mouseup", handleMouseUp);
	} else {
		console.log("Not an annotation click, context menu will show");
	}
}

// Clear all temporary annotations
function clearAnnotations() {
	console.log("Clearing all annotations");
	temporaryAnnotations.value = [];
	cleanupAnnotationDrawing();
	console.log("All annotations cleared");
}

function handlePreviousMove() {
	gamesStore.previousMove(props.boardId);
	emit("previousMove");
}

function handleNextMove() {
	gamesStore.nextMove(props.boardId);
	emit("nextMove");
}

function rotateBoard() {
	globalStore.uiStore.setWhiteOnSide();
}

function startResize(event: MouseEvent) {
	event.preventDefault();
	event.stopPropagation();

	const startX = event.clientX;

	const handleMouseMove = (event: MouseEvent) => {
		const deltaX = event.clientX - startX;

		// Scale down the deltaX to avoid resizing too quickly
		const scaleFactor = 0.15;
		const newSquareSize = Math.max(
			16,
			Math.min(96, squareSizePixels.value + deltaX * scaleFactor),
		);
		globalStore.uiStore.updateBoardSquareSize(newSquareSize);
	};

	const handleMouseUp = () => {
		window.removeEventListener("mousemove", handleMouseMove);
		window.removeEventListener("mouseup", handleMouseUp);
	};

	window.addEventListener("mousemove", handleMouseMove);
	window.addEventListener("mouseup", handleMouseUp);
}

// Keyboard event handling
function handleKeyDown(event: KeyboardEvent) {
	if (event.key === "Escape") {
		clearAnnotations();
	}
}

// Add keyboard event listeners
onMounted(() => {
	document.addEventListener("keydown", handleKeyDown);
});

// Cleanup on unmount
onUnmounted(() => {
	document.removeEventListener("keydown", handleKeyDown);
	cleanupAnnotationDrawing(); // Clean up any active drawing state
});
</script>

<!-- 
By jurgenwesterhof (adapted from work of Cburnett) - 
http://commons.wikimedia.org/wiki/Template:SVG_chess_pieces
CC BY-SA 3.0
Link: https://commons.wikimedia.org/w/index.php?curid=35634436
-->

