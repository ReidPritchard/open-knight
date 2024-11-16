import { invoke } from "@tauri-apps/api/core";
import { MOCKED, setupMocks } from "../test/mock";
import {
  apiExplorerStateToExplorerState,
  apiSelectedGameToGame,
} from "./api-conversions";
import type { IExplorerState } from "./types";

// Setup the API
setupMocks();

export default {
  /**
   * Get the explorer state from the backend.
   */
  getExplorerState: async () => {
    const state: string = await invoke("get_explorer_state");
    const parsedState = MOCKED ? state : apiExplorerStateToExplorerState(state);
    return parsedState as IExplorerState;
  },

  /**
   * Get the selected game from the backend.
   */
  getSelectedGame: async () => {
    const response: string = await invoke("get_selected_game");
    const game: string | null = response === "null" ? null : response;
    const parsedGame = apiSelectedGameToGame(game);
    return parsedGame;
  },

  /**
   * Parse a PGN text and update the games and selected game.
   */
  parsePgnText: async (pgnText: string) => {
    await invoke("parse_pgn", { pgn: pgnText });
  },

  /**
   * Empty the database.
   */
  emptyDatabase: async () => {
    await invoke("empty_db");
  },
};
