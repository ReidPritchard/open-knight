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
      :class="boardWhiteOrientation === 'top' ? 'rotate-180' : ''"
    >
      <div v-for="row in 8" :key="row" class="flex">
        <ChessBoardSquare
          v-for="col in 8"
          :key="col"
          :row="getLogicalPosition(row, col).row"
          :col="getLogicalPosition(row, col).col"
          :square-size="squareSizePixels"
          :piece="
            pieceAt(
              getLogicalPosition(row, col).row,
              getLogicalPosition(row, col).col
            )
          "
          :piece-image="
            getPieceImage(
              pieceAt(
                getLogicalPosition(row, col).row,
                getLogicalPosition(row, col).col
              ) ?? ''
            )
          "
          :can-move="
            canMovePiece(
              getLogicalPosition(row, col).row,
              getLogicalPosition(row, col).col
            )
          "
          :is-selected="
            isSelected(
              getLogicalPosition(row, col).row,
              getLogicalPosition(row, col).col
            )
          "
          :is-valid-move="
            isValidMove(
              getLogicalPosition(row, col).row,
              getLogicalPosition(row, col).col
            )
          "
          :is-highlighted="highlightCurrentMove(row, col)"
          @drop="
            handleDrop(
              getLogicalPosition(row, col).row,
              getLogicalPosition(row, col).col
            )
          "
          @click="
            handleSquareClick(
              $event,
              getLogicalPosition(row, col).row,
              getLogicalPosition(row, col).col
            )
          "
          @drag-start="
            handleDragStart(
              getLogicalPosition(row, col).row,
              getLogicalPosition(row, col).col
            )
          "
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
        {{
          currentMove?.game_move?.ply_number !== undefined
            ? `${Math.floor(currentMove.game_move.ply_number / 2) + 1}. ${
                currentMove.game_move.san
              }`
            : "N/A"
        }}
      </span>
      <button
        class="join-item btn"
        :disabled="
          currentMoveIndex === undefined ||
          currentMove?.children_ids.length === undefined ||
          currentMove?.children_ids.length === 0
        "
        @click="handleNextMove"
      >
        <PhArrowRight />
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { PhArrowLeft, PhArrowRight } from "@phosphor-icons/vue";
import { computed, onMounted, ref, watch } from "vue";
import api from "../../shared/api";
import type { ChessAnnotation } from "../../shared/bindings";
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

/**
 * Component props
 */
const props = defineProps<{
  boardId: number;
  squareSize?: number;
}>();

/**
 * Component emits
 */
const emit = defineEmits<{
  (e: "move", move: { from: string; to: string }): void;
  (e: "error", error: Error): void;
  (e: "previousMove"): void;
  (e: "nextMove"): void;
}>();

// Store access
const globalStore = useGlobalStore();
const gamesStore = globalStore.gamesStore;

// Board state from store
const boardState = computed(() => gamesStore.getBoardState(props.boardId));

const currentMove = computed(() => boardState.value?.currentMove);
const currentMoveIndex = computed(() => boardState.value?.currentMoveIndex);
const currentPosition = computed(() => boardState.value?.currentPosition);
const validMoves = computed(() => boardState.value?.validMoves);

// Board orientation
const boardWhiteOrientation = computed(
  () => globalStore.uiStore.boardWhiteOrientation
);

// Valid moves cache (`row,col` -> `{ row: number; col: number }[]`)
// row and col are 1-indexed (1-8)
const validPositionMoves = ref<
  Record<string, Array<{ row: number; col: number }>>
>({});
// Valid moves for the selected piece
const validPieceMoves = ref<Array<{ row: number; col: number }>>([]);

// Square size with default
const squareSizePixels = computed(() => props.squareSize || 64);

// Watch the current move to fetch valid moves
watch(currentMove, async () => {
  // First clear the cache to avoid stale data
  validPositionMoves.value = {};
  await fetchValidMoves();
});

/**
 * Convert visual board coordinates to logical coordinates
 * (1-8, 1-8) -> (0-7, 0-7)
 * Handles board orientation
 */
const getLogicalPosition = computed(() => {
  return (visualRow: number, visualCol: number) => {
    if (boardWhiteOrientation.value === "top") {
      return { row: 8 - visualRow, col: 8 - visualCol };
    }
    return { row: visualRow - 1, col: visualCol - 1 };
  };
});

/**
 * Convert logical coordinates to visual board coordinates
 * (0-7, 0-7) -> (1-8, 1-8)
 * Handles board orientation
 */
const getVisualPosition = computed(() => {
  return (logicalRow: number, logicalCol: number) => {
    if (boardWhiteOrientation.value === "top") {
      return { row: 8 - logicalRow, col: 8 - logicalCol };
    }
    return { row: logicalRow + 1, col: logicalCol + 1 };
  };
});

// Annotations processing
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

/**
 * Calculate arrow coordinates for annotations
 */
const arrowCoordinates = computed(() => {
  if (!annotations.value?.length) return null;

  const validArrows = annotations.value.filter(
    (annotation) => annotation.arrows
  );

  if (!validArrows.length || !validArrows[0].arrows) return null;

  const { arrows } = validArrows[0];
  const isRotated = boardWhiteOrientation.value === "top";

  return {
    from: calculateSquareCenter(
      algebraicToCoords(arrows.from).x,
      algebraicToCoords(arrows.from).y,
      squareSizePixels.value,
      isRotated
    ),
    to: calculateSquareCenter(
      algebraicToCoords(arrows.to).x,
      algebraicToCoords(arrows.to).y,
      squareSizePixels.value,
      isRotated
    ),
  };
});

/**
 * Get current board position in FEN notation
 */
const currentPositionFEN = computed(() => {
  // If there is a game selected and we are on a move, use that position
  if (currentPosition.value?.fen) {
    return { fen: currentPosition.value.fen };
  }

  // Default starting position
  const defaultFEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
  return { fen: defaultFEN };
});

/**
 * Get parsed board position
 */
const currentPositionParsed = computed(() => {
  if (!currentPositionFEN.value) return undefined;
  return parseFen(currentPositionFEN.value.fen);
});

/**
 * Get current turn from FEN
 */
const currentTurn = computed(() => {
  if (!currentPositionFEN.value) return "w";
  return currentPositionFEN.value.fen.split(" ")[1];
});

// Selected piece state
const selectedPiece = ref<{ row: number; col: number } | null>(null);

/**
 * Check if a square is selected
 */
const isSelected = (row: number, col: number) => {
  const selected =
    selectedPiece.value?.row === row && selectedPiece.value?.col === col;
  if (selected) {
    console.log("selected", row, col);
  }
  return selected;
};

/**
 * Check if the current move should be highlighted on this square
 */
const highlightCurrentMove = (row: number, col: number) => {
  if (!currentMove?.value?.game_move?.uci) return false;

  const algebraic = coordsToAlgebraic(col, row);
  const { from, to } = parseUciMove(currentMove.value.game_move.uci);

  return from === algebraic && to === algebraic;
};

/**
 * Handle drag start event
 */
const handleDragStart = async (row: number, col: number) => {
  console.log("handleDragStart", row, col);
  selectedPiece.value = { row, col };
  validPieceMoves.value = await getValidMoves(row, col);
};

/**
 * Handle drop event for drag and drop
 */
const handleDrop = async (row: number, col: number) => {
  if (!selectedPiece.value || !isValidMove.value(row, col)) return;

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
    console.error("Failed to make move:", error);
    emit("error", error instanceof Error ? error : new Error(String(error)));
  } finally {
    // Reset selection regardless of success/failure
    selectedPiece.value = null;
    validPieceMoves.value = [];
  }
};

/**
 * Handle square click event
 */
const handleSquareClick = async (
  event: MouseEvent,
  row: number,
  col: number
) => {
  event.preventDefault();

  if (isAnnotationClick(event)) {
    console.log("Annotation click - skipping");
    // TODO: Handle annotation
    return;
  }

  // Handle normal piece selection/move
  const piece = pieceAt(row, col);

  if (selectedPiece.value && isValidMove.value(row, col)) {
    // Move the selected piece to this square
    await handleDrop(row, col);
  } else if (piece && canMovePiece(row, col) && !isSelected(row, col)) {
    // Select this piece
    selectedPiece.value = { row, col };
    validPieceMoves.value = await getValidMoves(row, col);
  } else {
    // Deselect
    selectedPiece.value = null;
    validPieceMoves.value = [];
  }
};

/**
 * Handle previous move button click
 */
const handlePreviousMove = () => {
  gamesStore.previousMove(props.boardId);
  emit("previousMove");
};

/**
 * Handle next move button click
 */
const handleNextMove = () => {
  gamesStore.nextMove(props.boardId);
  emit("nextMove");
};

/**
 * Check if there are valid moves from the given square
 */
const canMovePiece = (row: number, col: number) => {
  // Check if it's the player's turn
  const piece = pieceAt(row, col);
  if (!piece) return false;

  const isWhite = isWhitePiece(piece);
  const isWhiteTurn = currentTurn.value === "w";

  if (isWhite !== isWhiteTurn) {
    // console.log(`Piece ${piece} is not ${isWhiteTurn ? "white" : "black"}`);
    return false;
  }

  // Check if the piece has valid moves
  // NOTE: `fetchValidMoves` must be called before this!!!
  const positionMoves = validPositionMoves.value[`${row},${col}`] || [];
  return positionMoves.length > 0;
};

/**
 * Check if the selected piece can move to the given square
 */
const isValidMove = computed(() => {
  return (row: number, col: number) => {
    return validPieceMoves.value.some(
      (move) => move.row === row && move.col === col
    );
  };
});

/**
 * Get valid moves for a piece
 */
const getValidMoves = async (row: number, col: number) => {
  const piece = pieceAt(row, col);
  if (!piece) return [];

  // Check cache first
  if (validPositionMoves.value[`${row},${col}`]) {
    return validPositionMoves.value[`${row},${col}`];
  }

  // Fetch if not cached
  await fetchValidMoves();
  return validPositionMoves.value[`${row},${col}`] || [];
};

/**
 * Fetch valid moves from API/store
 */
const fetchValidMoves = async () => {
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
    console.error("Failed to fetch valid moves:", error);
    emit("error", error instanceof Error ? error : new Error(String(error)));
  }
};

/**
 * Get the piece at a specific position
 */
const pieceAt = (row: number, col: number) => {
  const board = currentPositionParsed.value;
  if (!board) return undefined;

  const piece = board[row]?.[col];
  return isValidPiece(piece) ? piece : undefined;
};

/**
 * Get the image path for a piece
 */
const getPieceImage = (piece: string) => {
  return getPieceImagePath(piece);
};

// Setup
onMounted(async () => {
  try {
    await fetchValidMoves();
  } catch (error) {
    console.error("Error setting up the chess board:", error);
  }
});
</script>

<!-- 
By jurgenwesterhof (adapted from work of Cburnett) - 
http://commons.wikimedia.org/wiki/Template:SVG_chess_pieces
CC BY-SA 3.0
Link: https://commons.wikimedia.org/w/index.php?curid=35634436
-->
