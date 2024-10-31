import { Square } from "./types";

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

export const getSquareColor = (row: number, col: number): string => {
  return (row + col) % 2 === 0 ? "light-square" : "dark-square";
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

export const createBoard = (): Square[][] => {
  const rawBoard = initialBoard();
  return rawBoard.map((row, rowIndex) =>
    row.map((piece, colIndex) => ({
      row: rowIndex,
      col: colIndex,
      piece,
      isTarget: false,
    })),
  );
};

export const formatCoordinates = (square: Square): string => {
  const rank = String.fromCharCode(97 + square.col);
  const file = square.row + 1;
  return `${rank}, ${file}`;
};
