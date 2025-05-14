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
      <div v-for="row in 8" :key="row" class="flex">
        <ChessBoardSquare
          v-for="col in 8"
          :key="col"
          :row="row - 1"
          :col="col - 1"
          :square-size="squareSizePixels"
          :piece="getPieceAt(row, col)"
          :piece-image="getPieceImage(getPieceAt(row, col) ?? '')"
          :can-move="canMovePiece(row - 1, col - 1)"
          :is-selected="isSquareSelected(row - 1, col - 1)"
          :is-valid-move="isValidMoveTarget(row - 1, col - 1)"
          :is-highlighted="isPartOfCurrentMove(row, col)"
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
  algebraicToCoords,
  calculateSquareCenter,
  coordsToAlgebraic,
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
const validMoves = computed(() => boardState.value?.validMoves);

// Board orientation
const isBoardFlipped = computed(
  () => globalStore.uiStore.boardWhiteOrientation === "top"
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

// Valid moves cache (`row,col` -> `{ row: number; col: number }[]`)
// row and col are 1-indexed (1-8)
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

const currentTurn = computed(() =>
  currentPositionFEN.value ? currentPositionFEN.value.fen.split(" ")[1] : "w"
);

// ---------------
// Coordinate transformations
// ---------------
function getLogicalPosition(visualRow: number, visualCol: number) {
  // Convert from 1-indexed visual grid (1-8) to 0-indexed logical position (0-7)
  const row = visualRow - 1;
  const col = visualCol - 1;

  // No need to transform here - the CSS rotation handles the visual part,
  // and the logical positions should match the internal board array
  return { row, col };
}

// ---------------
// Piece and board utilities
// ---------------
function getPieceAt(row: number, col: number) {
  const board = currentPositionParsed.value;
  if (!board) return undefined;

  const { row: boardRow, col: boardCol } = getLogicalPosition(row, col);

  // Access the board array with the transformed coordinates
  const piece = board[boardRow]?.[boardCol];
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
  const isWhiteTurn = currentTurn.value === "w";

  if (isWhite !== isWhiteTurn) {
    return false;
  }

  // Check if the piece has valid moves
  const positionMoves = validPositionMoves.value[`${row},${col}`] || [];
  return positionMoves.length > 0;
}

function isPartOfCurrentMove(row: number, col: number) {
  if (!currentMove?.value?.game_move?.uci) return false;

  const { from, to } = parseUciMove(currentMove.value.game_move.uci);

  const boardRow = 7 - row + 1;
  const boardCol = col - 1;

  // Convert to algebraic notation for comparison
  const algebraic = coordsToAlgebraic(boardCol, boardRow);
  return from === algebraic || to === algebraic;
}

// ---------------
// Annotations
// ---------------
const annotations = computed(
  () =>
    boardState.value?.currentMove?.game_move?.annotations?.map((annotation) => {
      const { comment, arrows, highlights } = annotation;
      return {
        comment,
        arrows: arrows ? parseUciMove(arrows) : null,
        highlights: highlights ? algebraicToCoords(highlights) : null,
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
      algebraicToCoords(arrows.from).x,
      algebraicToCoords(arrows.from).y,
      squareSizePixels.value,
      isBoardFlipped.value
    ),
    to: calculateSquareCenter(
      algebraicToCoords(arrows.to).x,
      algebraicToCoords(arrows.to).y,
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

  // Check cache first
  if (validPositionMoves.value[`${row},${col}`]) {
    return validPositionMoves.value[`${row},${col}`];
  }

  // Fetch if not cached
  await fetchValidMoves();
  return validPositionMoves.value[`${row},${col}`] || [];
}

async function fetchValidMoves() {
  try {
    const moves = validMoves.value || [];
    validPositionMoves.value = moves.reduce((acc, move) => {
      const { from, to } = parseUciMove(move.uci);
      const { x: fromX, y: fromY } = algebraicToCoords(from);
      const { x: toX, y: toY } = algebraicToCoords(to);
      const fromKey = `${fromY},${fromX}`;

      // Create the key if it doesn't exist
      acc[fromKey] = acc[fromKey] || [];

      // Add the move to the key
      acc[fromKey].push({ row: toY, col: toX });
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

  const toSquare = coordsToAlgebraic(col, row);
  const fromSquare = coordsToAlgebraic(
    selectedPiece.value.col,
    selectedPiece.value.row
  );
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

async function handleSquareClick(event: MouseEvent, row: number, col: number) {
  event.preventDefault();

  if (isAnnotationClick(event)) {
    // Handle annotation click (future enhancement)
    return;
  }

  // Handle normal piece selection/move
  const piece = getPieceAt(row, col);

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
  globalStore.uiStore.boardWhiteOrientation = isBoardFlipped.value
    ? "bottom"
    : "top";
}

function startResize(event: MouseEvent) {
  event.preventDefault();
  event.stopPropagation();

  const startX = event.clientX;

  const handleMouseMove = (event: MouseEvent) => {
    const deltaX = event.clientX - startX;

    // Scale down the deltaX/deltaY to avoid resizing too quickly
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
