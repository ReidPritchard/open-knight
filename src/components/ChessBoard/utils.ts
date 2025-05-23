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
 * Converts algebraic notation (e.g., "e4") to screen/board coordinates
 * Coordinates use 0-7 for both row and column, with (0,0) at top-left (a8)
 * @param square The algebraic notation of the square
 * @returns Object with screen/board coordinates (row, col)
 */
export const algebraicToBoard = (
	square: string,
): { row: number; col: number } => {
	return {
		col: square.charCodeAt(0) - 97, // 'a' = 0, 'b' = 1, etc.
		row: 8 - Number.parseInt(square.charAt(1), 10), // '8' = 0, '7' = 1, etc.
	};
};

/**
 * Converts screen/board coordinates to algebraic notation
 * @param row Screen/board row (0-7, 0 = top)
 * @param col Screen/board column (0-7, 0 = left)
 * @returns The algebraic notation of the square (e.g., "e4")
 */
export const boardToAlgebraic = (row: number, col: number): string => {
	return `${String.fromCharCode(97 + col)}${8 - row}`;
};

/**
 * Calculates pixel coordinates for the center of a square
 * @param col Column index (0-7)
 * @param row Row index (0-7)
 * @param squareSize Size of each square in pixels
 * @param isRotated Whether the board is rotated (white on top)
 * @returns Object with x and y pixel coordinates
 */
export const calculateSquareCenter = (
	col: number,
	row: number,
	squareSize: number,
	isRotated: boolean,
): { x: number; y: number } => {
	const boardSize = squareSize * 8;
	const centerOffset = squareSize / 2;

	let x = col * squareSize + centerOffset;
	let y = row * squareSize + centerOffset; // Simplified: row directly maps to y-coordinate

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
 * @returns The board state, with 0,0 being a8 (top-left) and 7,7 being h1 (bottom-right)
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
		parsedBoard.push(rowSquares); // Add each row directly (no need to reverse)
	}

	return parsedBoard;
};

export type BoardState = string[][];

/**
 * Check if a piece is white
 * @param piece Chess piece character
 * @returns True if the piece is white (uppercase)
 */
export const isWhitePiece = (piece: string) => piece === piece.toUpperCase();
