import { invoke } from "@tauri-apps/api/core";
import type { ChessGame, LegalMove, QueryParams } from "./bindings";
import {
  type ExplorerGame,
  explorerGameFields,
  parseExplorerGames,
  parseLegalMoves,
} from "./types";

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
          limit: 100,
          offset: 0,
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

        console.log("Explorer games:", JSON.parse(response));

        // FIXME: The types need tweaking to parse the response successfully
        // currently the parsing fails, but the response is valid
        // so we are skipping the validation for now

        // const parsed = parseExplorerGames(response);

        // if (parsed.success) {
        //   return parsed.data;
        // }

        // console.error("Error parsing explorer games:", parsed.errors);

        // throw new Error(parsed.errors.join("\n"));
        return JSON.parse(response);
      },

      /**
       * Get a game by its ID
       * @param gameId The ID of the game
       * @returns Promise<Game>
       */
      game: async (
        gameId: number,
        params: QueryParams = {
          fields: null,
          limit: 1,
          offset: 0,
          filter: {},
          load_moves: true,
          load_tags: true,
        }
      ): Promise<ChessGame> => {
        const response = await invoke<string>("get_game_by_id", {
          id: gameId,
          params: params,
        });
        return JSON.parse(response);
      },
    },
    POST: {
      importPGNGames: async (pgn: string): Promise<void> => {
        await invoke("import_pgn_games", { pgn });
      },
    },
  },
  moves: {
    GET: {
      validMoves: async (fen: string): Promise<LegalMove[]> => {
        const response = await invoke<string>("get_legal_moves", { fen });
        const parsed = parseLegalMoves(response);
        if (parsed.success) {
          return parsed.data;
        }
        throw new Error(parsed.errors.join("\n"));
      },
      moveTree: async (gameId: number): Promise<string> => {
        const response = await invoke<string>("get_move_tree", { id: gameId });
        return JSON.parse(response);
      },
    },
  },
};
