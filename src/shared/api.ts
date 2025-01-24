import { invoke } from "@tauri-apps/api/core";
import type { AllValidMoves } from "./bindings/AllValidMoves";
import { parseAllValidMoves } from "./types";

export default {
  /**
   * Parse a PGN text and update the games and selected game.
   * @param pgnText The PGN text to parse
   */
  parsePgnText: async (pgnText: string): Promise<void> => {
    const result = await invoke("parse_pgn", { pgn: pgnText });
    console.log("Parse PGN result:", result);
  },

  /**
   * Empty the database and reset the app state.
   */
  emptyDatabase: async (): Promise<void> => {
    await invoke("empty_db");
  },

  /**
   * Get all valid moves for a given position.
   * @param position The FEN string of the position
   * @returns Promise<Array<{ row: number; col: number }>> Array of valid moves
   */
  getAllValidMoves: async (position?: string): Promise<AllValidMoves> => {
    if (!position) {
      throw new Error("No position provided");
    }

    const response = await invoke("get_all_valid_moves", { position });
    const parsed = parseAllValidMoves(response);
    if (parsed.success) {
      console.log("Parsed valid moves:", parsed.data);
      return parsed.data;
    }
    throw new Error("Invalid response from backend");
  },

  /**
   * Make a move in the backend.
   * @param position The FEN string of the position
   * @param move The move to make
   * @returns Promise<string> The new position
   */
  makeMove: async (position: string, move: string): Promise<string> => {
    return await invoke("make_move", { position, move });
  },
};
