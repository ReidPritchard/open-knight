/**
 * Converts a piece letter to its corresponding image filename
 * @param piece The piece letter (e.g., 'p', 'N', 'k', etc.)
 * @returns The path to the piece's image file
 */
export const getPieceImagePath = (piece: string): string => {
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
  return `/${isWhite ? "white" : "black"}_${pieceName}.svg`;
};

/**
 * Checks if a piece letter represents a valid chess piece
 * @param piece The piece letter to validate
 * @returns True if the piece is a valid chess piece letter
 */
export const isValidPiece = (piece: string | undefined): boolean => {
  return (
    typeof piece === "string" &&
    ["p", "n", "r", "k", "q", "b"].includes(piece.toLowerCase())
  );
};

/**
 * Converts algebraic notation (e.g., "e4") to board coordinates
 * @param square The algebraic notation of the square
 * @returns Object with x (0-7) and y (0-7) coordinates
 */
export const algebraicToBoard = (
  square: string
): { col: number; row: number } => {
  return {
    col: square.charCodeAt(0) - 97, // 'a' = 0, 'b' = 1, etc.
    row: 8 - Number.parseInt(square.charAt(1), 10), // '1' = 7, '2' = 6, etc.
  };
};

/**
 * Converts board coordinates to algebraic notation
 * @param x File index (0-7)
 * @param y Rank index (0-7)
 * @returns The algebraic notation of the square (e.g., "e4")
 */
export const boardToAlgebraic = (x: number, y: number): string => {
  return `${String.fromCharCode(97 + x)}${8 - y}`;
};

/**
 * Calculates pixel coordinates for the center of a square
 * @param file File index (0-7)
 * @param rank Rank index (1-8)
 * @param squareSize Size of each square in pixels
 * @param isRotated Whether the board is rotated (white on top)
 * @returns Object with x and y pixel coordinates
 */
export const calculateSquareCenter = (
  file: number,
  rank: number,
  squareSize: number,
  isRotated: boolean
): { x: number; y: number } => {
  const boardSize = squareSize * 8;
  const centerOffset = squareSize / 2;

  let x = file * squareSize + centerOffset;
  let y = (8 - rank) * squareSize + centerOffset;

  if (isRotated) {
    x = boardSize - x;
    y = boardSize - y;
  }

  return { x, y };
};

/**
 * Checks if a click event is for an annotation
 * @param event The mouse event
 * @returns True if the click is meant for annotation (right click, shift+click, ctrl+click, or cmd+click)
 */
export const isAnnotationClick = (event: MouseEvent): boolean => {
  return event.button === 2 || event.shiftKey || event.ctrlKey || event.metaKey;
};

/**
 * Splits a UCI notation move into its from and to squares
 * @param move The UCI notation move
 * @returns An object with from and to squares
 */
export const parseUciMove = (move: string): { from: string; to: string } => {
  const from = move.slice(0, 2);
  const to = move.slice(2, 4);

  return { from, to };
};

/**
 * Parses a FEN string into a board state
 * @param fen The FEN string
 * @returns The board state
 */
export const parseFen = (fen: string): BoardState => {
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
};

export type BoardState = string[][];

/**
 * Check if a piece is white
 */
export const isWhitePiece = (piece: string) => piece === piece.toUpperCase();

/**
 * Convert board coordinates to screen coordinates, accounting for board orientation
 * @param row Board row (0-7, 0 = top row)
 * @param col Board column (0-7, 0 = leftmost column)
 * @param isFlipped Whether the board is visually flipped
 * @returns Screen coordinates (after accounting for board orientation)
 */
export const boardToScreen = (
  row: number,
  col: number,
  isFlipped: boolean
): { row: number; col: number } => {
  if (isFlipped) {
    return { row: 7 - row, col: 7 - col };
  }
  return { row, col };
};

/**
 * Convert screen coordinates to board coordinates, accounting for board orientation
 * @param row Screen row (0-7)
 * @param col Screen column (0-7)
 * @param isFlipped Whether the board is visually flipped
 * @returns Board coordinates (logical position in the board array)
 */
export const screenToBoard = (
  row: number,
  col: number,
  isFlipped: boolean
): { row: number; col: number } => {
  if (isFlipped) {
    return { row: 7 - row, col: 7 - col };
  }
  return { row, col };
};
