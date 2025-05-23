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
      <div v-for="row in 8" :key="row - 1" class="flex">
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
          @drag-start="handleDragStart(row - 1, col - 1)"
        />
      </div>
    </div>

    <!-- Annotation Arrow -->
    <div v-if="arrowCoordinates" class="absolute inset-0">
      <AnnotationArrow
        :from="arrowCoordinates.from"
        :to="arrowCoordinates.to"
        :options="{ color: 'yellow', size: 5 }"
      />
    </div>
  </div>

  <!-- Move Navigation -->
  <div class="flex flex-row items-center justify-center mt-4">
    <div class="join">
      <button
        class="join-item btn"
        :disabled="currentMoveIndex === -1"
        @click="handlePreviousMove"
      >
        <PhArrowLeft />
      </button>
      <span class="label join-item px-8 w-40">
        {{ formatCurrentMove }}
      </span>
      <button
        class="join-item btn"
        :disabled="!hasNextMove"
        @click="handleNextMove"
      >
        <PhArrowRight />
      </button>
    </div>

    <!-- Rotate Board -->
    <button class="btn btn-sm ml-4" @click="rotateBoard">
      <PhArrowsClockwise size="16" />
    </button>

    <!-- Resize Board (drag to resize) -->
    <div class="ml-4 cursor-ew-resize" @mousedown="startResize">
      <PhArrowsOutLineHorizontal size="16" />
    </div>
  </div>
</template>

<script setup lang="ts">
import {
  PhArrowLeft,
  PhArrowRight,
  PhArrowsClockwise,
  PhArrowsOutLineHorizontal,
} from "@phosphor-icons/vue";
import { computed, onMounted, ref, watch } from "vue";
import api from "../../shared/api";
import { useGlobalStore } from "../../stores/";
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
const currentMove = computed(() => boardState.value?.currentMove);
const currentMoveIndex = computed(() => boardState.value?.currentMoveIndex);
const currentPosition = computed(() => boardState.value?.currentPosition);
const currentTurn = computed(() => boardState.value?.currentTurn);
const validMoves = computed(() => boardState.value?.validMoves);

// Board orientation
const isBoardFlipped = computed(
  () => globalStore.uiStore.whiteOnSide === "top"
);

// Board styling
const squareSizePixels = computed(() => globalStore.uiStore.boardSquareSize);
const boardTheme = computed(() => globalStore.uiStore.boardTheme);

// Move navigation
const hasNextMove = computed(
  () =>
    currentMove.value?.children_ids?.length &&
    currentMove.value?.children_ids?.length > 0
);

const formatCurrentMove = computed(() => {
  if (!currentMove.value?.game_move?.ply_number) return "N/A";

  const moveNumber = Math.floor(currentMove.value.game_move.ply_number / 2) + 1;
  return `${moveNumber}. ${currentMove.value.game_move.san}`;
});

// Selected piece state
const selectedPiece = ref<{ row: number; col: number } | null>(null);
const validPieceMoves = ref<Array<{ row: number; col: number }>>([]);

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
// Board state
// ---------------
const currentPositionFEN = computed(() => {
  if (currentPosition.value?.fen) {
    return { fen: currentPosition.value.fen };
  }
  // Default starting position
  return { fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" };
});

const currentPositionParsed = computed(() =>
  currentPositionFEN.value ? parseFen(currentPositionFEN.value.fen) : undefined
);

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
  const board = currentPositionParsed.value;
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
    (move) => move.row === row && move.col === col
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
const annotations = computed(
  () =>
    currentMove.value?.game_move?.annotations?.map((annotation) => {
      const { comment, arrows, highlights } = annotation;
      return {
        comment,
        arrows: arrows ? parseUciMove(arrows) : null,
        highlights: highlights ? algebraicToBoard(highlights) : null,
      };
    }) || []
);

const arrowCoordinates = computed(() => {
  if (!annotations.value?.length) return null;

  const validArrows = annotations.value.filter(
    (annotation) => annotation.arrows
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
      isBoardFlipped.value
    ),
    to: calculateSquareCenter(
      toCoords.col,
      toCoords.row,
      squareSizePixels.value,
      isBoardFlipped.value
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
    selectedPiece.value.col
  );
  const toSquare = boardToAlgebraic(row, col);
  const moveNotation = fromSquare + toSquare;

  try {
    await gamesStore.makeMove(props.boardId, moveNotation);
    gamesStore.nextMove(props.boardId);
  } catch (error) {
    console.error("Error making move:", error);
    emit("error", error instanceof Error ? error : new Error(String(error)));
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
async function handleSquareClick(event: MouseEvent, row: number, col: number) {
  event.preventDefault();

  if (isAnnotationClick(event)) {
    // Handle annotation click (future enhancement)
    return;
  }

  const piece = getPieceAtCoords(row, col);

  if (selectedPiece.value && isValidMoveTarget(row, col)) {
    // Move the selected piece to this square
    await handleDrop(row, col);
  } else if (piece && canMovePiece(row, col) && !isSquareSelected(row, col)) {
    // Select this piece
    selectedPiece.value = { row, col };
    validPieceMoves.value = getValidMoves(row, col);
  } else {
    // Deselect
    selectedPiece.value = null;
    validPieceMoves.value = [];
  }
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
      Math.min(96, squareSizePixels.value + deltaX * scaleFactor)
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

// ---------------
// Setup
// ---------------
</script>

<!-- 
By jurgenwesterhof (adapted from work of Cburnett) - 
http://commons.wikimedia.org/wiki/Template:SVG_chess_pieces
CC BY-SA 3.0
Link: https://commons.wikimedia.org/w/index.php?curid=35634436
-->
