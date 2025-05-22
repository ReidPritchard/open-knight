import typia from "typia";
import type { ChessGame, LegalMove } from "./bindings";

////////////////////////////////////////////////////////////
// Application UI Interfaces
////////////////////////////////////////////////////////////

// Moved to `themes.ts` as it was only theme related,
// generally types/constants could use some better organization
// but that's a 'todo' for another day

////////////////////////////////////////////////////////////
// Application Interfaces
// Api interfaces are mostly sub-sets of backend models (bindings)
////////////////////////////////////////////////////////////

/**
 * Fields of `ChessGame` model that are displayed in the explorer/library view
 */
export const explorerGameFields: (keyof ChessGame)[] = [
  "id",
  "white_player",
  "black_player",
  "tournament",
  "opening",
  "result",
  "round",
  "date",
  "tags",
] as const;

/**
 * A game displayed in the explorer/library view
 */
export type ExplorerGame = Pick<ChessGame, (typeof explorerGameFields)[number]>;

/**
 * Parse a JSON string into an `ExplorerGame` object
 */
export const parseExplorerGame = typia.json.createValidateParse<ExplorerGame>();

/**
 * Parse a JSON string into an array of `ExplorerGame` objects
 */
export const parseExplorerGames =
  typia.json.createValidateParse<ExplorerGame[]>();

/**
 * Parse a JSON string into an array of `LegalMove` objects
 */
export const parseLegalMoves = typia.json.createValidateParse<LegalMove[]>();

/**
 * Events emitted by the engine
 */
export interface EngineEvents {
  analysisUpdate: AnalysisUpdate;
  bestMove: BestMove;
}

export interface BestMove {
  move: string;
  ponder?: string;
  timestamp: number;
}

/**
 * Best move of the engine's analysis
 */
export type BestMovePayload = [move: string, ponder?: string];

/**
 * Score of the engine's analysis
 */
export interface Score {
  value: number;
  type?: "centipawns" | "mate";
}

/**
 * Update of the engine's analysis
 */
export interface AnalysisUpdate {
  depth?: number;
  seldepth?: number;
  time?: number;
  nodes?: number;
  pv?: string[];
  multipv?: number;
  score?: Score;
  hashfull?: number;
  nps?: number;
  tbhits?: number;
}

/**
 * Analysis update event payload
 */
export type AnalysisUpdatePayload = [
  engineName: string,
  update:
    | {
        AnalysisUpdate: AnalysisUpdate;
      }
    | {
        BestMove: BestMovePayload;
      }
];

/**
 * Parse a JSON string into an `AnalysisUpdatePayload` object
 */
export const parseAnalysisUpdatePayload =
  typia.json.createValidateParse<AnalysisUpdatePayload>();

/**
 * Engine option
 */
export interface EngineOption {
  type: "check" | "spin" | "combo" | "button" | "string";
  value?: string;
  default?: string;
  min?: number;
  max?: number;
  var?: string[];
}

/**
 * Engine options
 */
export interface EngineSettings {
  [name: string]: EngineOption;
}

/**
 * Engine settings event payload
 */
export type EngineSettingsPayload = [
  engineName: string,
  EngineSettings: EngineSettings
];

/**
 * Parse a JSON string into an `EngineSettingsPayload` object
 */
export const parseEngineSettingsPayload =
  typia.json.createValidateParse<EngineSettingsPayload>();

////////////////////////////////////////////////////////////
// Error Types
////////////////////////////////////////////////////////////

/**
 * Error thrown when interacting with the engine
 */
export class EngineError extends Error {
  public readonly EngineError: string;

  constructor(message: string) {
    super(message);
    this.EngineError = message;
  }
}
