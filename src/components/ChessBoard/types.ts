import typia from "typia";

export interface Square {
  row: number;
  col: number;

  piece: string | null;

  /**
   * If the square is a target square for a move
   * only used when a piece is selected or being dragged
   */
  isTarget: boolean;
}

export type Board = Square[][];

export interface Move {
  from: { row: number; col: number };
  to: { row: number; col: number };
  piece: string;
}

export interface Arrow {
  from: { row: number; col: number };
  to: { row: number; col: number };
  color: string;
}

/**
 * An array of arrows
 * Associated with a single board position
 */
export type Arrows = Arrow[];

export type Orientation = "white" | "black";

export type Animation = "none" | "slow" | "medium" | "fast";

/**
 * How coordinates are displayed on the board
 */
export const CoordinatesStyle = {
  none: "none",
  inside: "inside",
  outside: "outside",
  verbose: "verbose",
} as const;
export type CoordinatesStyleType =
  (typeof CoordinatesStyle)[keyof typeof CoordinatesStyle];
export const validateCoordinatesStyle =
  typia.createValidate<CoordinatesStyleType>();

/**
 * Visual design/UI of board
 */
export type BoardStyle = {
  /**
   * The board's colors
   */
  colors: {
    lightSquare: string;
    darkSquare: string;
    /**
     * The color of the border around each square
     */
    squareBorder: string;
  };
  /**
   * The size of the border around each square
   */
  squareBorderWidth: number;
  /**
   * Piece style
   */
  pieceStyle: {
    /**
     * Sprite sheet image of the pieces
     * If not provided, the pieces will be rendered using unicode characters
     */
    spriteSheetImage?: string;
    /**
     * The colors of the unicode pieces
     */
    unicodeColors: {
      white: string;
      black: string;
    };
  };
};

export const boardStyleColorPresets = {
  wood: {
    colors: {
      lightSquare: "#f0d9b5",
      darkSquare: "#b58863",
      squareBorder: "#000",
    },
  },
  blue: {
    colors: {
      lightSquare: "#a5c4d4",
      darkSquare: "#7DAAC0",
      squareBorder: "#000",
    },
  },
  // TODO: Add more presets
};
