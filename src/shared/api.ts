import { invoke } from "@tauri-apps/api/core";
import { MOCKED, setupMocks } from "../test/mock";
import {
  apiExplorerStateToExplorerGames,
  apiSelectedGameToGame,
} from "./api-conversions";
import type { ExplorerGame } from "./bindings/ExplorerGame";

// Setup the API
setupMocks();

export default {
  /**
   * Get the explorer state from the backend.
   * @throws {Error} If the backend returns invalid data
   * @returns Promise<ExplorerGame[]> Array of explorer games
   */
  getExplorerState: async (): Promise<ExplorerGame[]> => {
    const serializedState: string = await invoke("get_explorer_state");
    if (MOCKED) {
      return serializedState as unknown as ExplorerGame[];
    }

    const parsed = JSON.parse(serializedState);

    console.log("Parsed explorer state:", parsed);

    return apiExplorerStateToExplorerGames(serializedState);
  },

  /**
   * Get the selected game from the backend.
   * @throws {Error} If the backend returns invalid data
   * @returns Promise<APIGame | null> Selected game or null if none selected
   */
  getSelectedGame: async () => {
    const serializedGame: string = await invoke("get_selected_game");
    if (serializedGame === "null") {
      return null;
    }

    const parsed = JSON.parse(serializedGame);
    if (typeof parsed !== "object") {
      throw new Error("Invalid game data received from backend");
    }

    console.log("Parsed selected game:", parsed);

    return apiSelectedGameToGame(serializedGame);
  },

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
};
