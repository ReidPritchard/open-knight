import typia from "typia";

/**
 * Types for the application layout
 */
export interface IWindow {
  id: string;
  size: number;
  resizable?: boolean;
  collapsed?: boolean;
}
export const assertWindow = typia.createAssert<IWindow>();
export const validateWindow = typia.createValidate<IWindow>();

export enum WindowDisplay {
  /**
   * Displays one tab at a time and has a tab bar at the top
   */
  Tabs = "tabs",
  /**
   * Displays multiple panes at a time with a splitter bar between them
   * Can be resized
   * TODO: Prevent overflow
   * TODO: If horizontal, when collapsed, display a vertical "section" bar (opposite if vertical)
   */
  Split = "split",
  /**
   * Displays one section at a time but all section headers are visible and collapsible
   */
  Accordion = "accordion",
  /**
   * Attach a window to the side of the current window
   * When collapsed, displays a thin bar on the opposite side
   * Should not be displayed as a direct child of another window
   */
  Panel = "panel",
}

export enum WindowDirection {
  Horizontal = "horizontal",
  Vertical = "vertical",
}

export enum PanelPosition {
  Left = "left",
  Right = "right",
  Top = "top",
  Bottom = "bottom"
}

export interface IWindowContainer extends IWindow {
  /**
   * The display mode of the window container
   * tabs: displays one tab at a time and has a tab bar at the top
   * split: displays multiple panes at a time
   * accordion: displays one section at a time but all section headers are visible and collapsible
   */
  display: WindowDisplay;
  direction: WindowDirection;
  panelPosition?: PanelPosition;
  minSize?: number;
  maxSize?: number;
  children: ILayout[];
}
export const assertWindowContainer = typia.createAssert<IWindowContainer>();
export const validateWindowContainer = typia.createValidate<IWindowContainer>();

export type ILayout = IWindow | IWindowContainer;

export interface IMove {
  id: number; // Corresponds to i32 in Rust
  game_id: number; // Corresponds to i32 in Rust
  move_number: number; // Corresponds to i32 in Rust
  move_san: string; // Corresponds to String in Rust
  variation_id: number | null; // Corresponds to Option<i32> in Rust
  parent_variation_id: number | null; // Corresponds to Option<i32> in Rust
  fen: string | null; // Corresponds to Option<String> in Rust
  annotation: string | null; // Corresponds to Option<String> in Rust
}
export const assertMove = typia.createAssert<IMove>();
export const validateMove = typia.createValidate<IMove>();
export const parseMove = typia.json.createValidateParse<IMove>();

export interface IGame {
  id: number;
  headers: Record<string, string>;
  pgn: string;
  moves: IMove[];
  errors: string[];
}
export const assertGame = typia.createAssert<IGame>();
export const validateGame = typia.createValidate<IGame>();
export const parseGame = typia.json.createValidateParse<IGame>();

export interface IExplorerState {
  games: IGame[];
}
export const assertExplorerState = typia.createAssert<IExplorerState>();
export const validateExplorerState = typia.createValidate<IExplorerState>();
export const parseExplorerState =
  typia.json.createValidateParse<IExplorerState>();
