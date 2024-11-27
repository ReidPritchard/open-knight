import type { BoardStyle, Square } from "./types";

export const pieceUnicode: { [key: string]: string } = {
  wK: "♔",
  wQ: "♕",
  wR: "♖",
  wB: "♗",
  wN: "♘",
  wP: "♙",
  bK: "♚",
  bQ: "♛",
  bR: "♜",
  bB: "♝",
  bN: "♞",
  bP: "♟",
};

const getSquareColor = (row: number, col: number): string => {
  return (row + col) % 2 === 0 ? "light-square" : "dark-square";
};

const getSquareColorKey = (color: string): "lightSquare" | "darkSquare" =>
  color === "light-square" ? "lightSquare" : "darkSquare";

export const getSquareStyle = (square: Square, style: BoardStyle): string => {
  const styles: string[] = [];

  // Square color
  const color = getSquareColor(square.row, square.col);
  const backgroundColor = style.colors[getSquareColorKey(color)];
  if (backgroundColor) {
    styles.push(`background-color: ${backgroundColor}`);
  }

  // Square border
  if (style.squareBorderWidth && style.colors.squareBorder) {
    styles.push(
      `border: ${style.squareBorderWidth}px solid ${style.colors.squareBorder}`
    );
  }

  // TODO: Piece Sprite

  // Piece color
  if (square.piece) {
    const pieceColor = square.piece[0] === "w" ? "white" : "black";
    const pieceColorValue = style.pieceStyle?.unicodeColors?.[pieceColor];
    if (pieceColorValue) {
      styles.push(`color: ${pieceColorValue}`);
    }
  }

  return styles.join("; ");
};

export const initialBoard = (): (string | null)[][] => {
  return [
    ["bR", "bN", "bB", "bQ", "bK", "bB", "bN", "bR"],
    ["bP", "bP", "bP", "bP", "bP", "bP", "bP", "bP"],
    [null, null, null, null, null, null, null, null],
    [null, null, null, null, null, null, null, null],
    [null, null, null, null, null, null, null, null],
    [null, null, null, null, null, null, null, null],
    ["wP", "wP", "wP", "wP", "wP", "wP", "wP", "wP"],
    ["wR", "wN", "wB", "wQ", "wK", "wB", "wN", "wR"],
  ];
};

export const parseFenToBoard = (fen: string): Square[][] => {
  if (fen === "start") {
    return createBoard();
  }

  // Split the FEN string into its components
  const parts = fen.trim().split(/\s+/);
  // if (parts.length !== 6) {
  //   throw new Error("Invalid FEN string: Incorrect number of fields.");
  // }

  const [
    boardNotation,
    activeColor,
    castlingAvailability,
    enPassantTarget,
    halfMoveClock,
    fullMoveNumber,
  ] = parts;

  const ranks = boardNotation.split("/");
  if (ranks.length !== 8) {
    throw new Error("Invalid FEN string: Incorrect number of ranks.");
  }

  const board: Square[][] = [];

  // Process each rank from top (rank 8) to bottom (rank 1)
  for (let rankIndex = 0; rankIndex < 8; rankIndex++) {
    const rankStr = ranks[rankIndex];
    const row: Square[] = [];
    let fileIndex = 0;

    // Convert rankIndex to row number (0 = rank 1, 7 = rank 8)
    const rowNumber = 7 - rankIndex;

    for (const char of rankStr) {
      if (/[1-8]/.test(char)) {
        const emptySquares = Number.parseInt(char, 10);
        for (let i = 0; i < emptySquares; i++) {
          row.push({
            row: rowNumber,
            col: fileIndex,
            piece: null,
            isTarget: false,
          });
          fileIndex++;
        }
      } else if (/[prnbqkPRNBQK]/.test(char)) {
        const isBlack = char === char.toLowerCase();
        const pieceColor = isBlack ? "b" : "w";
        const pieceType = char.toUpperCase();
        const piece = pieceColor + pieceType;

        row.push({
          row: rowNumber,
          col: fileIndex,
          piece,
          isTarget: false,
        });
        fileIndex++;
      } else {
        throw new Error(
          `Invalid FEN string: Invalid character '${char}' in board notation.`
        );
      }
    }

    if (row.length !== 8) {
      throw new Error(
        `Invalid FEN string: Incorrect number of squares in rank ${
          8 - rankIndex
        }.`
      );
    }

    board[rowNumber] = row;
  }

  // Parse the additional FEN context if needed

  // Optionally, log or process the context
  // console.log(context);

  return board;
};

export const createBoard = (): Square[][] => {
  const rawBoard = initialBoard();
  return rawBoard.map((row, rowIndex) =>
    row.map((piece, colIndex) => ({
      row: rowIndex,
      col: colIndex,
      piece,
      isTarget: false,
    }))
  );
};

export const formatCoordinates = (square: Square): string => {
  const rank = String.fromCharCode(97 + square.col);
  const file = square.row + 1;
  return `${rank}, ${file}`;
};
