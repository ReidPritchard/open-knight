import { invoke } from "@tauri-apps/api/core";
import { apiExplorerStateToExplorerGames } from "./api-conversions";
import type { ExplorerGame } from "./bindings/ExplorerGame";

export default {
  /**
   * Get the explorer state from the backend.
   * @throws {Error} If the backend returns invalid data
   * @returns Promise<ExplorerGame[]> Array of explorer games
   */
  getExplorerState: async (): Promise<ExplorerGame[]> => {
    const serializedState: string = await invoke("get_explorer_state");
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
    console.log("Fetching selected game from API");
    const serializedGame: string = await invoke("get_selected_game");
    console.log("Raw API response:", serializedGame);

    if (serializedGame === "null") {
      console.log("No game selected");
      return null;
    }

    const parsed = JSON.parse(serializedGame);
    if (typeof parsed !== "object") {
      console.error("Invalid game data received from backend");
      throw new Error("Invalid game data received from backend");
    }

    console.log("Parsed game data:", parsed);
    return parsed;
  },

  /**
   * Set the selected game in the backend.
   * @param gameId The ID of the game to set as selected, or null to clear selection
   */
  setSelectedGame: async (gameId: number | null): Promise<void> => {
    console.log("Setting selected game to:", gameId);
    await invoke("set_selected_game", { gameId });
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
