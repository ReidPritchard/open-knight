import typia from "typia";
import type { ChessGame, LegalMove } from "./bindings";

////////////////////////////////////////////////////////////
// Application UI Interfaces
////////////////////////////////////////////////////////////

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
