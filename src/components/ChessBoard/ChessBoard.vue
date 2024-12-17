<template>
  <div class="grid grid-rows-8 grid-flow-col">
    <div v-for="row in 8" :key="row" class="flex">
      <div
        v-for="col in 8"
        :key="col"
        :class="`w-16 h-16 ${
          (row + col) % 2 === 0 ? 'bg-slate-400' : 'bg-slate-600'
        } ${isValidMove(row - 1, col - 1) ? 'bg-green-400/50' : ''} 
        flex items-center justify-center relative`"
        @dragover.prevent
        @drop="handleDrop(row - 1, col - 1)"
        @click="handleSquareClick(row - 1, col - 1)"
      >
        <template v-if="pieceAt(row - 1, col - 1) !== undefined">
          <img
            :src="getPieceImage(pieceAt(row - 1, col - 1) ?? '')"
            :class="`w-12 h-12 select-none ${
              isSelected(row - 1, col - 1) ? 'ring-2 ring-yellow-400' : ''
            }`"
            :draggable="canMovePiece(row - 1, col - 1)"
            @dragstart="handleDragStart(row - 1, col - 1)"
            alt="Chess piece"
          />
        </template>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from "vue";
import { useGameStore } from "../../stores/game";

// FIXME: We should either use backend API or chess.js for this
// no point in re-implementing the wheel ;)

const gameStore = useGameStore();

const selectedGame = computed(() => gameStore.selectedGame);
const currentMove = computed(() => gameStore.currentMove);

// The current position from which we want to display the board
const currentPosition = computed(() => {
  if (currentMove.value?.parent_position) {
    return currentMove.value.parent_position;
  }
  return selectedGame.value?.moves[0].parent_position;
});

// State for drag and drop
const selectedPiece = ref<{ row: number; col: number } | null>(null);
const validMoves = ref<Array<{ row: number; col: number }>>([]);

// Get the current turn (white or black) from FEN
const currentTurn = computed(() => {
  if (!currentPosition.value) return "w";
  return currentPosition.value.fen.split(" ")[1];
});

const isWhitePiece = (piece: string) => piece === piece.toUpperCase();

const canMovePiece = (row: number, col: number) => {
  const piece = pieceAt(row, col);
  if (!piece) return false;
  // Only allow moving pieces of the current turn's color
  return (currentTurn.value === "w") === isWhitePiece(piece);
};

const isSelected = (row: number, col: number) => {
  return selectedPiece.value?.row === row && selectedPiece.value?.col === col;
};

const isValidMove = (row: number, col: number) => {
  return validMoves.value.some((move) => move.row === row && move.col === col);
};

const handleDragStart = (row: number, col: number) => {
  if (!canMovePiece(row, col)) return;
  selectedPiece.value = { row, col };
  // TODO: Calculate valid moves based on piece type and position
  validMoves.value = calculateValidMoves(row, col);
};

const handleDrop = (row: number, col: number) => {
  if (!selectedPiece.value || !isValidMove(row, col)) return;

  // TODO: Update game state with the move
  console.log(
    `Move from (${selectedPiece.value.row}, ${selectedPiece.value.col}) to (${row}, ${col})`
  );

  // Reset selection
  selectedPiece.value = null;
  validMoves.value = [];
};

const handleSquareClick = (row: number, col: number) => {
  const piece = pieceAt(row, col);

  if (selectedPiece.value) {
    if (isValidMove(row, col)) {
      handleDrop(row, col);
    } else {
      selectedPiece.value = null;
      validMoves.value = [];
    }
  } else if (piece && canMovePiece(row, col)) {
    selectedPiece.value = { row, col };
    validMoves.value = calculateValidMoves(row, col);
  }
};

const calculateValidMoves = (row: number, col: number) => {
  const piece = pieceAt(row, col);
  if (!piece) return [];

  const moves: Array<{ row: number; col: number }> = [];
  const isWhite = isWhitePiece(piece);
  const pieceType = piece.toLowerCase();

  // Helper function to check if a square is empty or contains an enemy piece
  const isValidTarget = (r: number, c: number) => {
    if (r < 0 || r > 7 || c < 0 || c > 7) return false;
    const targetPiece = pieceAt(r, c);
    return !targetPiece || isWhitePiece(targetPiece) !== isWhite;
  };

  // Helper function to add moves in a direction until blocked
  const addMovesInDirection = (
    rowDir: number,
    colDir: number,
    maxSteps = 8
  ) => {
    let r = row + rowDir;
    let c = col + colDir;
    let steps = 0;

    while (r >= 0 && r < 8 && c >= 0 && c < 8 && steps < maxSteps) {
      const targetPiece = pieceAt(r, c);
      if (!targetPiece) {
        moves.push({ row: r, col: c });
      } else {
        if (isWhitePiece(targetPiece) !== isWhite) {
          moves.push({ row: r, col: c });
        }
        break;
      }
      r += rowDir;
      c += colDir;
      steps++;
    }
  };

  switch (pieceType) {
    case "p": // Pawn
      const direction = isWhite ? -1 : 1; // White pawns move up (-1), black pawns move down (+1)
      const startRow = isWhite ? 6 : 1;

      // Forward move
      if (!pieceAt(row + direction, col)) {
        moves.push({ row: row + direction, col });
        // Initial two-square move
        if (row === startRow && !pieceAt(row + 2 * direction, col)) {
          moves.push({ row: row + 2 * direction, col });
        }
      }

      // Captures
      for (const captureCol of [col - 1, col + 1]) {
        if (captureCol >= 0 && captureCol < 8) {
          const targetPiece = pieceAt(row + direction, captureCol);
          if (targetPiece && isWhitePiece(targetPiece) !== isWhite) {
            moves.push({ row: row + direction, col: captureCol });
          }
        }
      }
      break;

    case "n": // Knight
      const knightMoves = [
        [-2, -1],
        [-2, 1],
        [-1, -2],
        [-1, 2],
        [1, -2],
        [1, 2],
        [2, -1],
        [2, 1],
      ];
      for (const [dr, dc] of knightMoves) {
        if (isValidTarget(row + dr, col + dc)) {
          moves.push({ row: row + dr, col: col + dc });
        }
      }
      break;

    case "b": // Bishop
      addMovesInDirection(-1, -1); // Up-left
      addMovesInDirection(-1, 1); // Up-right
      addMovesInDirection(1, -1); // Down-left
      addMovesInDirection(1, 1); // Down-right
      break;

    case "r": // Rook
      addMovesInDirection(-1, 0); // Up
      addMovesInDirection(1, 0); // Down
      addMovesInDirection(0, -1); // Left
      addMovesInDirection(0, 1); // Right
      break;

    case "q": // Queen (combines bishop and rook moves)
      addMovesInDirection(-1, -1); // Up-left
      addMovesInDirection(-1, 1); // Up-right
      addMovesInDirection(1, -1); // Down-left
      addMovesInDirection(1, 1); // Down-right
      addMovesInDirection(-1, 0); // Up
      addMovesInDirection(1, 0); // Down
      addMovesInDirection(0, -1); // Left
      addMovesInDirection(0, 1); // Right
      break;

    case "k": // King
      for (let dr = -1; dr <= 1; dr++) {
        for (let dc = -1; dc <= 1; dc++) {
          if (dr === 0 && dc === 0) continue;
          if (isValidTarget(row + dr, col + dc)) {
            moves.push({ row: row + dr, col: col + dc });
          }
        }
      }
      break;
  }

  return moves;
};

const pieceAt = (row: number, col: number) => {
  if (!currentPosition.value) return undefined;
  const fen = currentPosition.value.fen;
  // The FEN is typically "FEN_position side_to_move castling_available en_passant_halfmoves_fullmoves"
  const board = fen.split(" ")[0];
  const boardArray = board.split("/");

  // Each element of boardArray is a FEN rank from top (8) to bottom (1)
  const fenRow = boardArray[row];

  if (!fenRow) return undefined;

  // Decode this FEN row into an array of exactly 8 squares
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

  const piece = rowSquares[col];
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
</script>

<!-- 
By jurgenwesterhof (adapted from work of Cburnett) - 
http://commons.wikimedia.org/wiki/Template:SVG_chess_pieces
CC BY-SA 3.0
Link: https://commons.wikimedia.org/w/index.php?curid=35634436
-->
