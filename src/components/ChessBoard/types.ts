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
