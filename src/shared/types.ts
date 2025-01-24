import typia from "typia";

////////////////////////////////////////////////////////////
// Application UI Interfaces
////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////
// Application Interfaces
// Api interfaces are mostly sub-sets of columns in the database
////////////////////////////////////////////////////////////

/**
 * A game displayed in the explorer/library view
 * Subset of `ChessGame` model
 */
export interface ExplorerGame {
  id: string;
}

// Type Guards
export const isExplorerGame = typia.createIs<ExplorerGame>();
export const isAPIGame = typia.createIs<APIGame>();
export const isAllValidMoves = typia.createIs<AllValidMoves>();

// Parsers
export const parseFullGame = typia.createValidate<FullGame>();
export const parseAPIGame = typia.createValidate<APIGame>();
export const parseAllValidMoves = typia.createValidate<AllValidMoves>();
