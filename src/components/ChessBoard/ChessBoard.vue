<template>
  <div
    class="grid grid-rows-8 grid-flow-col transition-all duration-100"
    :class="boardWhiteOrientation === 'top' ? 'rotate-180' : ''"
  >
    <div v-for="row in 8" :key="row" class="flex">
      <div
        v-for="col in 8"
        :key="col"
        :class="`w-16 h-16 ${
          (row + col) % 2 === 0 ? 'bg-slate-400' : 'bg-slate-600'
        } flex items-center justify-center relative`"
        @drop="handleDrop(8 - row, 8 - col)"
        @click="handleSquareClick(8 - row, 8 - col)"
      >
        <template v-if="pieceAt(8 - row, 8 - col) !== undefined">
          <img
            :src="getPieceImage(pieceAt(8 - row, 8 - col) ?? '')"
            :class="`w-12 h-12 select-none ${
              isSelected(8 - row, 8 - col) ? 'ring-2 ring-primary' : ''
            }`"
            :draggable="canMovePiece(8 - row, 8 - col)"
            @dragstart="handleDragStart(8 - row, 8 - col)"
            alt="Chess piece"
          />
        </template>
        <template v-if="isValidMove(8 - row, 8 - col)">
          <div
            class="absolute inset-0 bg-success/80 w-1/5 h-1/5 m-auto rounded-full"
          ></div>
        </template>
      </div>
    </div>
  </div>

  <div class="flex flex-col items-center justify-center">
    <span class="badge badge-primary">
      {{ JSON.stringify(currentMove) }}
    </span>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { useGlobalStore } from "../../stores/";
import api from "../../shared/api";

const props = defineProps<{
  boardId: number;
}>();

const globalStore = useGlobalStore();

// Get board-specific store interface once
const boardStore = computed(() =>
  globalStore.gamesStore.getBoardStore(props.boardId)
);

// Use the board-specific store for all getters
const selectedGame = computed(() => boardStore.value.getActiveGame());
const currentMove = computed(() => boardStore.value.getCurrentMove());
const nextMoves = computed(() => boardStore.value.getNextMoves());

// UI store access remains global since it's shared
const boardWhiteOrientation = computed(
  () => globalStore.uiStore.boardWhiteOrientation
);

// The current position from which we want to display the board
const currentPositionFEN = computed(() => {
  // If there is a game selected and we are on a move, use that position
  if (currentMove.value?.position) {
    return currentMove.value.position;
  }
  // if a game is selected, but we haven't started "moving" yet, use the starting position
  if (selectedGame.value?.game) {
    return { fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" };
  }

  // if no game is selected, use the standard starting position
  return { fen: "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1" };
});

const currentPositionParsed = computed(() => {
  if (!currentPositionFEN.value) return undefined;
  const fen = currentPositionFEN.value.fen;
  // Get the board part of the FEN (before the first space)
  const board = fen.split(" ")[0];
  const boardArray = board.split("/");

  // Create an 8x8 2D array representing the board
  const parsedBoard: string[][] = [];

  // Process each row in the FEN (from 8th rank to 1st rank)
  for (const fenRow of boardArray) {
    const rowSquares: string[] = [];
    for (const char of fenRow) {
      if (/\d/.test(char)) {
        // A digit indicates that many consecutive empty squares
        const emptyCount = Number.parseInt(char, 10);
        for (let i = 0; i < emptyCount; i++) {
          rowSquares.push("");
        }
      } else {
        // A letter indicates a piece
        rowSquares.push(char);
      }
    }
    parsedBoard.unshift(rowSquares); // Add each row to the beginning instead of the end
  }

  return parsedBoard;
});

// State for drag and drop
const selectedPiece = ref<{ row: number; col: number } | null>(null);
// The valid moves in the current position (contains all valid moves for all valid pieces)
// This lets us only fetch it once. Maps row, col to a list of valid moves from that square/piece
const validPositionMoves = ref<
  Record<string, Array<{ row: number; col: number }>>
>({});

// These are the valid moves for the currently selected piece (actively displayed)
const validPieceMoves = ref<Array<{ row: number; col: number }>>([]);

// Get the current turn (white or black) from FEN
const currentTurn = computed(() => {
  if (!currentPositionFEN.value) return "w";
  return currentPositionFEN.value.fen.split(" ")[1];
});

const isWhitePiece = (piece: string) => piece === piece.toUpperCase();

const isSelected = (row: number, col: number) => {
  return selectedPiece.value?.row === row && selectedPiece.value?.col === col;
};

const handleDragStart = async (row: number, col: number) => {
  if (!canMovePiece(row, col)) return;
  selectedPiece.value = { row, col };
  validPieceMoves.value = await getValidMoves(row, col);
};

const handleDrop = async (row: number, col: number) => {
  if (!selectedPiece.value || !isValidMove.value(row, col)) return;

  // Convert row/col to algebraic notation (e.g. "e4")
  const toSquare = `${String.fromCharCode(97 + col)}${row + 1}`;
  const fromSquare = `${String.fromCharCode(97 + selectedPiece.value.col)}${
    selectedPiece.value.row + 1
  }`;
  const moveNotation = `${fromSquare}${toSquare}`;

  try {
    // Make the move in the backend
    await api.makeMove(currentPositionFEN.value.fen, moveNotation);
  } catch (error) {
    console.error("Failed to make move:", error);
  }

  // Reset selection
  selectedPiece.value = null;
  validPieceMoves.value = [];
};

const handleSquareClick = async (row: number, col: number) => {
  const piece = pieceAt(row, col);

  if (selectedPiece.value && isValidMove.value(row, col)) {
    handleDrop(row, col);
  } else if (piece && canMovePiece(row, col) && !isSelected(row, col)) {
    // If we click a piece that is already selected, unselect it
    // If we click a different piece, select it
    // I think this should work since you can't eat your own pieces
    // I also feel like there is probably an edge case I'm not thinking of
    selectedPiece.value = { row, col };
    validPieceMoves.value = await getValidMoves(row, col);
  } else {
    selectedPiece.value = null;
    validPieceMoves.value = [];
  }
};

/**
 * Check if there are valid moves *from* the given square
 * @param row The row of the source square
 * @param col The column of the source square
 * @returns True if there are valid moves for the piece at the given square
 */
const canMovePiece = (row: number, col: number) => {
  return validPositionMoves.value[`${row},${col}`] !== undefined;
};

/**
 * Check if the selected piece can move *to* the given square
 * @param row The row of the target square
 * @param col The column of the target square
 * @returns True if the move is valid for the currently selected piece
 */
const isValidMove = computed(() => {
  return (row: number, col: number) => {
    return validPieceMoves.value.some(
      (move) => move.row === row && move.col === col
    );
  };
});

const getValidMoves = async (row: number, col: number) => {
  const piece = pieceAt(row, col);
  if (!piece) return [];

  // Check if we have valid moves for this piece in the current position
  if (validPositionMoves.value[`${row},${col}`]) {
    return validPositionMoves.value[`${row},${col}`];
  }

  // If not, fetch them from the backend
  await fetchValidMoves();

  return validPositionMoves.value[`${row},${col}`];
};

const fetchValidMoves = async () => {
  try {
    // For now, we'll rely on the store's validMoves
    // Later we can add a proper API endpoint for this
    return nextMoves.value ?? [];
  } catch (error) {
    console.error("Failed to fetch valid moves:", error);
    return [];
  }
};

const pieceAt = (row: number, col: number) => {
  const board = currentPositionParsed.value;
  if (!board) return undefined;

  const piece = board[row]?.[col];
  // Return the piece if it is recognized; otherwise undefined
  if (!piece || !["p", "n", "r", "k", "q", "b"].includes(piece.toLowerCase())) {
    return undefined;
  }

  return piece;
};

const getPieceImage = (piece: string) => {
  // Determine if piece is white or black by case (uppercase = white, lowercase = black)
  const isWhite = piece === piece.toUpperCase();

  // Map piece letters to their names
  const pieceNames = {
    p: "pawn",
    n: "knight",
    r: "rook",
    k: "king",
    q: "queen",
    b: "bishop",
  };

  const pieceName = pieceNames[piece.toLowerCase() as keyof typeof pieceNames];
  // Assuming you have placed your images in `public` folder as `white_*.svg` and `black_*.svg`
  return `/${isWhite ? "white" : "black"}_${pieceName}.svg`;
};

onMounted(async () => {
  await fetchValidMoves();
});
</script>

<!-- 
By jurgenwesterhof (adapted from work of Cburnett) - 
http://commons.wikimedia.org/wiki/Template:SVG_chess_pieces
CC BY-SA 3.0
Link: https://commons.wikimedia.org/w/index.php?curid=35634436
-->
