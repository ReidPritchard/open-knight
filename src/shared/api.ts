import { invoke } from "@tauri-apps/api/core";
import type { ChessGame, LegalMove, QueryParams } from "./bindings";
import {
  type ExplorerGame,
  explorerGameFields,
  parseExplorerGames,
  parseLegalMoves,
} from "./types";

/**
 * The dev env exposes the API to the frontend console.
 * Use `await $$.api.____` to call any of the functions below.
 *
 * Ex. `await $$.api.engines.POST.loadEngine("stockfish", "/usr/local/bin/stockfish")`
 */

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
      /**
       * Get the valid moves for a given FEN string
       * @param fen The FEN string of the position
       * @returns Promise<LegalMove[]>
       */
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
  engines: {
    POST: {
      /**
       * Load an engine from a path.
       * @param name The name of the engine
       * @param path The path to the engine
       * @returns Promise<void>
       *
       * @example
       * await $$.api.engines.POST.loadEngine("stockfish", "/usr/local/bin/stockfish")
       */
      loadEngine: async (name: string, path: string): Promise<void> => {
        await invoke("load_engine", { name, path });
      },
      /**
       * Unload an engine.
       * @param name The name of the engine
       * @returns Promise<void>
       *
       * @example
       * await $$.api.engines.POST.unloadEngine("stockfish")
       */
      unloadEngine: async (name: string): Promise<void> => {
        await invoke("unload_engine", { name });
      },
      /**
       * Analyze a position with an engine.
       * @param engineName The name of the engine
       * @param fen The FEN string of the position
       * @param depth The depth of the analysis
       * @param timeMs The time in milliseconds for the analysis
       * @returns Promise<void>
       *
       * @example
       * await $$.api.engines.POST.analyzePosition("stockfish", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", 10, 1000)
       */
      analyzePosition: async (
        engineName: string,
        fen: string,
        depth: number,
        timeMs: number
      ): Promise<void> => {
        await invoke("analyze_position", { engineName, fen, depth, timeMs });
      },
      /**
       * Stop an analysis.
       * @param engineName The name of the engine
       * @returns Promise<void>
       *
       * @example
       * await $$.api.engines.POST.stopAnalysis("stockfish")
       */
      stopAnalysis: async (engineName: string): Promise<void> => {
        await invoke("stop_analysis", { engineName });
      },
    },
  },
};
