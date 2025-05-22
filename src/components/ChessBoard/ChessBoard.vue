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

// Valid moves cache
// Key format: "row,col" where row and col are 0-based indices
// FIXME: This pattern of updating the position, fetching valid moves, and then
// creating a map/cache of valid moves is not great. It's probably worth revisiting
// and redesigning to avoid async race conditions or unnecessary re-renders.
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
  const positionMoves = validPositionMoves.value[`${row},${col}`] || [];
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
async function getValidMoves(row: number, col: number) {
  const piece = getPieceAtCoords(row, col);
  if (!piece) return [];

  // Check cache first using coordinates
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

      // Convert algebraic notation to coordinates
      const { row: fromRow, col: fromCol } = algebraicToBoard(from);
      const { row: toRow, col: toCol } = algebraicToBoard(to);

      // Create a cache entry using coordinates
      const fromKey = `${fromRow},${fromCol}`;

      // Create the key if it doesn't exist
      acc[fromKey] = acc[fromKey] || [];

      // Add the move to the key
      acc[fromKey].push({ row: toRow, col: toCol });
      return acc;
    }, {} as Record<string, Array<{ row: number; col: number }>>);

    // Log the cache (for debugging)
    console.group("Valid moves cache");
    for (const key in validPositionMoves.value) {
      const [row, col] = key.split(",").map(Number);
      console.log(
        `${boardToAlgebraic(row, col)}: ${validPositionMoves.value[key].length}`
      );
    }
    console.groupEnd();
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
    validPieceMoves.value = await getValidMoves(row, col);
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
