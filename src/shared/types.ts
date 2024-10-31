import typia from "typia";


/**
 * Mixin interface for collapsible windows/containers (for compositional purposes).
 */
export interface ICollapsible {
  /**
   * Determines if the window/container is collapsible.
   */
  collapsible: boolean;
  /**
   * Indicates if the window/container is collapsed.
   */
  collapsed: boolean;
}

/**
 * Mixin interface for resizable windows/containers.
 */
export interface IResizable {
  /**
   * Determines if the window/container is resizable.
   */
  resizable: boolean;
  /**
   * Minimum and maximum size constraints.
   */
  minSize?: number;
  maxSize?: number;
}

/**
 * Mixin interface for visibility of windows/containers.
 */
export interface IVisible {
  /**
   * Determines if the window/container is visible.
   */
  visible: boolean;

  /**
   * Determines if the window/container is closable.
   */
  closable: boolean;
}

/**
 * Base interface for all windows and containers.
 */
export interface IBaseWindow extends ICollapsible, IResizable, IVisible {
  id: string;
  title?: string;
  icon?: string;
  /**
   * Relative size of the window/container.
   * Used when 'fixedSize' is not specified.
   */
  size?: number;
  /**
   * Fixed size of the window/container in pixels.
   * If specified, the window/container has a fixed size and is not resizable.
   */
  fixedSize?: number;
}

/**
 * Interface for leaf windows that contain actual content.
 */
export interface IWindow extends IBaseWindow {
  /**
   * The component or content to render inside the window.
   */
  contentComponent: string;
}
export const assertWindow = typia.createAssert<IWindow>();
export const validateWindow = typia.createValidate<IWindow>();

/**
 * Directions for split views and panel positions.
 */
export enum LayoutDirection {
  Horizontal = "horizontal",
  Vertical = "vertical",
}

/**
 * Positions where a panel can be attached.
 */
export enum PanelPosition {
  Left = "left",
  Right = "right",
  Top = "top",
  Bottom = "bottom",
}

export enum WindowDisplayMode {
  /**
   * A simple container that fills the available space.
   * Allows for Left, Right, Top, and Bottom panels which
   * have their own content. Any non-panel children will be passed through
   * to the "content" slot.
   */
  Simple = "simple",
  /**
   * A container that displays its children in tabs.
   */
  Tabs = "tabs",
  /**
   * A flexible container that displays its children in either
   * vertical or horizontal, collapsible "blocks"/sub-windows
   */
  Flexible = "flexible",
}

/**
 * Base interface for window containers.
 */
export interface IWindowContainerBase extends IBaseWindow {
  display: WindowDisplayMode;
  /**
   * Child windows (blocks) within the container.
   */
  children: ILayout[];
}

/**
 * Interface for simple containers with optional left, right, top, and bottom panels.
 */
export interface ISimpleContainer extends IWindowContainerBase {
  display: WindowDisplayMode.Simple;

  /**
   * Left panel content.
   */
  left?: IWindowContainer;

  /**
   * Right panel content.
   */
  right?: IWindowContainer;

  /**
   * Top panel content.
   */
  top?: IWindowContainer;

  /**
   * Bottom panel content.
   */
  bottom?: IWindowContainer;
}
export const assertSimpleContainer = typia.createAssert<ISimpleContainer>();
export const validateSimpleContainer = typia.createValidate<ISimpleContainer>();

/**
 * Interface for Tab Containers.
 */
export interface ITabContainer extends IWindowContainerBase {
  display: WindowDisplayMode.Tabs;
  /**
   * ID of the currently active tab.
   */
  activeTabId?: string;
}

export const assertTabContainer = typia.createAssert<ITabContainer>();
export const validateTabContainer = typia.createValidate<ITabContainer>();

/**
 * Interface for flexible containers that display their children in
 * vertical or horizontal, collapsible "blocks"/sub-windows.
 */
export interface IFlexibleContainer extends IWindowContainerBase {
  display: WindowDisplayMode.Flexible;

  /**
   * Orientation of the flexible container.
   */
  orientation: LayoutDirection;
}
export const assertFlexibleContainer = typia.createAssert<IFlexibleContainer>();
export const validateFlexibleContainer = typia.createValidate<IFlexibleContainer>();

/**
 * Container interface that can represent different display modes.
 */
export type IWindowContainer = ISimpleContainer | ITabContainer | IFlexibleContainer;

export const validateWindowContainer = typia.createValidate<IWindowContainer>();

/**
 * Layout type that can be a window or any container.
 */
export type ILayout = IWindow | IWindowContainer;


/**
 * Interface for a chess move.
 */
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
