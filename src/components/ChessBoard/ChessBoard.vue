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
          :piece="getPieceAt(8 - row, col - 1)"
          :piece-image="getPieceImage(getPieceAt(8 - row, col - 1) ?? '')"
          :can-move="canMovePiece(8 - row, col - 1)"
          :is-selected="isSquareSelected(8 - row, col - 1)"
          :is-valid-move="isValidMoveTarget(8 - row, col - 1)"
          :is-highlighted="isPartOfCurrentMove(row - 1, col - 1)"
          :is-board-flipped="isBoardFlipped"
          :board-theme="boardTheme"
          :class="{ 'rotate-180': isBoardFlipped, 'rotate-0': !isBoardFlipped }"
          @drop="handleDrop(8 - row, col - 1)"
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

// Valid moves cache
// Key format: "row,col" where row and col are 0-based indices
const validPositionMoves = ref<
  Record<string, Array<{ row: number; col: number }>>
>({});

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
 * Gets a piece at the specified board coordinates
 * @param row 0-based row index (0-7)
 * @param col 0-based column index (0-7)
 * @returns The piece at the specified position, or undefined if none
 */
function getPieceAt(row: number, col: number) {
  const board = currentPositionParsed.value;
  if (!board) return undefined;

  // Access the board array directly with 0-based coordinates
  const piece = board[row]?.[col];
  return isValidPiece(piece) ? piece : undefined;
}

function getPieceImage(piece: string) {
  return getPieceImagePath(piece);
}

function isSquareSelected(row: number, col: number) {
  return selectedPiece.value?.row === row && selectedPiece.value?.col === col;
}

function isValidMoveTarget(row: number, col: number) {
  return validPieceMoves.value.some(
    (move) => move.row === row && move.col === col
  );
}

function canMovePiece(row: number, col: number) {
  // Check if it's the player's turn
  const piece = getPieceAt(row, col);
  if (!piece) return false;

  const isWhite = isWhitePiece(piece);
  const isWhiteTurn = currentTurn.value === "white";

  if (isWhite !== isWhiteTurn) {
    console.log("Not your turn:", piece, "from", row, col);
    return false;
  }

  // Check if the piece has valid moves
  const positionMoves = validPositionMoves.value[`${row},${col}`] || [];
  const canMove = positionMoves.length > 0;

  if (canMove) {
    console.log("Can move:", piece, "from", row, col, "to", positionMoves);
  } else {
    console.log("Cannot move:", piece);
    console.log(validPositionMoves.value);
  }

  return canMove;
}

/**
 * Checks if the current square is part of the current move
 * @param row 0-based row index, rendered coordinate (not board coordinates)
 * @param col 0-based column index
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
  // Calculate the square centers based on the current board orientation
  return {
    from: calculateSquareCenter(
      algebraicToBoard(arrows.from).col,
      algebraicToBoard(arrows.from).row,
      squareSizePixels.value,
      isBoardFlipped.value
    ),
    to: calculateSquareCenter(
      algebraicToBoard(arrows.to).col,
      algebraicToBoard(arrows.to).row,
      squareSizePixels.value,
      isBoardFlipped.value
    ),
  };
});

// ---------------
// Move handling
// ---------------
async function getValidMoves(row: number, col: number) {
  const piece = getPieceAt(row, col);
  if (!piece) return [];

  // Check cache first using 0-based coordinates
  const cacheKey = `${row},${col}`;
  if (validPositionMoves.value[cacheKey]) {
    return validPositionMoves.value[cacheKey];
  }

  // Fetch if not cached
  await fetchValidMoves();
  return validPositionMoves.value[cacheKey] || [];
}

async function fetchValidMoves() {
  try {
    const moves = validMoves.value || [];
    validPositionMoves.value = moves.reduce((acc, move) => {
      const { from, to } = parseUciMove(move.uci);

      // Convert algebraic notation to board coordinates
      const fromCoords = algebraicToBoard(from);
      const toCoords = algebraicToBoard(to);

      // Create a cache entry using 0-based coordinates
      const fromKey = `${fromCoords.row},${fromCoords.col}`;

      // Create the key if it doesn't exist
      acc[fromKey] = acc[fromKey] || [];

      // Add the move to the key
      acc[fromKey].push({ row: toCoords.row, col: toCoords.col });
      return acc;
    }, {} as Record<string, Array<{ row: number; col: number }>>);
  } catch (error) {
    emit("error", error instanceof Error ? error : new Error(String(error)));
  }
}

// ---------------
// Event handlers
// ---------------
async function handleDragStart(row: number, col: number) {
  selectedPiece.value = { row, col };
  validPieceMoves.value = await getValidMoves(row, col);
}

async function handleDrop(row: number, col: number) {
  if (!selectedPiece.value || !isValidMoveTarget(row, col)) return;

  // Convert board coordinates to algebraic notation
  const fromSquare = boardToAlgebraic(
    selectedPiece.value.row,
    selectedPiece.value.col
  );
  const toSquare = boardToAlgebraic(row, col);
  const moveNotation = fromSquare + toSquare;

  try {
    await api.makeMove(currentPositionFEN.value.fen, moveNotation);
    emit("move", { from: fromSquare, to: toSquare });
  } catch (error) {
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
 * @param row The row of the square visual coordinates (0-7)
 * @param col The column of the square visual coordinates (0-7)
 */
async function handleSquareClick(event: MouseEvent, row: number, col: number) {
  event.preventDefault();

  if (isAnnotationClick(event)) {
    // Handle annotation click (future enhancement)
    return;
  }

  // Handle normal piece selection/move

  // convert visual coordinates to board coordinates
  const boardRow = 7 - row;
  const boardCol = col;

  const piece = getPieceAt(boardRow, boardCol);

  if (selectedPiece.value && isValidMoveTarget(boardRow, boardCol)) {
    const fromNotation = boardToAlgebraic(
      7 - selectedPiece.value.row,
      selectedPiece.value.col
    );
    const toNotation = boardToAlgebraic(boardRow - 1, boardCol);
    console.log("Move:", fromNotation, "->", toNotation);

    // Move the selected piece to this square
    await handleDrop(boardRow, boardCol);
  } else if (
    piece &&
    canMovePiece(boardRow, boardCol) &&
    !isSquareSelected(boardRow, boardCol)
  ) {
    // Select this piece
    selectedPiece.value = { row: boardRow, col: boardCol };
    validPieceMoves.value = await getValidMoves(boardRow, boardCol);
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
watch(currentMove, async () => {
  // Reset the move cache when the current move changes
  validPositionMoves.value = {};
  await fetchValidMoves();
});

onMounted(async () => {
  try {
    await fetchValidMoves();
  } catch (error) {
    emit("error", error instanceof Error ? error : new Error(String(error)));
  }
});
</script>

<!-- 
By jurgenwesterhof (adapted from work of Cburnett) - 
http://commons.wikimedia.org/wiki/Template:SVG_chess_pieces
CC BY-SA 3.0
Link: https://commons.wikimedia.org/w/index.php?curid=35634436
-->
