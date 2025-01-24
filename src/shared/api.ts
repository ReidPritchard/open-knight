import { invoke } from "@tauri-apps/api/core";
import {
  type ExplorerGame,
  explorerGameFields,
  parseExplorerGames,
} from "./types";
import type { QueryParams } from "./bindings";

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
   * Make a move in the backend.
   * @param position The FEN string of the position
   * @param move The move to make
   * @returns Promise<string> The new position
   */
  makeMove: async (position: string, move: string): Promise<string> => {
    return await invoke("make_move", { position, move });
  },

  games: {
    GET: {
      /**
       * Get a list of games to display in the explorer view
       * @param params The query parameters (optional)
       * @returns Promise<ExplorerGame[]>
       * @throws Error if the response is not a valid ExplorerGame[]
       */
      explorer: async (
        params: Omit<QueryParams, "fields" | "load_tags"> = {
          limit: BigInt(100),
          offset: BigInt(0),
          filter: {},
          load_moves: false,
        }
      ): Promise<ExplorerGame[]> => {
        const response = await invoke<string>("query_games", {
          params: {
            ...params,
            fields: explorerGameFields,
            load_tags: true,
          },
        });

        const parsed = parseExplorerGames(response);

        if (parsed.success) {
          return parsed.data;
        }

        throw new Error(parsed.errors.join("\n"));
      },
    },
    POST: {
      importDemoGames: async (): Promise<void> => {
        await invoke("import_demo_games");
      },
    },
  },
};
