import typia from "typia";
import type { IExplorerState, IGame, IMove } from "./types";
import { assertGame } from "./types";

/**
 * This is an intermediate type that matches the raw json response from the tauri api
 * It should not be used outside of this file as it's difficult to work with
 */
interface PartialExplorerState {
  games: {
    id: number;
    // ex. [["Event", "Casual Game"], ["Result", "1-0"], ["White", "Thanos"], ["Black", "Thanos"]]
    headers: [string, string][];
    pgn: string;
    moves: IMove[];
    errors: string[];
  }[];
}
const parsePartialExplorerState =
  typia.json.createValidateParse<PartialExplorerState>();
const parsePartialGame =
  typia.json.createValidateParse<PartialExplorerState["games"][number]>();

/**
 * Convert an incoming "GameResult" (as defined in src-tauri/src/state.rs) to a "Game" (as defined in src/App.vue)
 */
export const gameResultToGame = (
  game: PartialExplorerState["games"][number]
): IGame => {
  console.log("Converting game result to game:", game.id); // Log the game ID being processed

  console.log("Parsing headers");
  const parsed_headers: Record<string, string> = {};
  const headers = game.headers;
  for (const header_index in headers) {
    // header is ["key", "value"]
    const [key, value] = headers[header_index];
    parsed_headers[key.toLowerCase()] = value;
  }

  console.log("Asserting game");
  const gameResult = assertGame({
    id: game.id,
    headers: parsed_headers,
    pgn: game.pgn,
    moves: game.moves,
    errors: game.errors,
  });
  console.log("Game result:", gameResult);
  return gameResult;
};

/**
 * Convert an api response "get_explorer_state" (JSON string) to an ExplorerState (as defined in src/App.vue)
 */
export function apiExplorerStateToExplorerState(
  apiExplorerState: string
): IExplorerState {
  console.log("Parsing API Explorer State"); // Log the start of parsing
  const parsed = parsePartialExplorerState(apiExplorerState);

  console.log("Checking success");
  if (parsed.success) {
    console.log("Converting games");
    // Convert each game to a IGame
    const games = parsed.data.games.map(gameResultToGame);
    const explorerState: IExplorerState = { games };
    console.log("Returning explorer state:", explorerState);
    return explorerState;
  }
  console.error(
    "Error parsing explorer state:",
    JSON.stringify(parsed.errors, null, 2)
  );
  // TODO: Better error handling, maybe a toast?
  throw new Error(parsed.errors.join("\n"));
}

/**
 * Api response "get_selected_game" (JSON string) to a Game (as defined in src/App.vue)
 */
export function apiSelectedGameToGame(
  apiSelectedGame: string | null
): IGame | null {
  console.log("Parsing API Selected Game"); // Log the start of parsing
  if (apiSelectedGame !== null) {
    const parsed = parsePartialGame(apiSelectedGame);
    if (parsed.success) {
      return gameResultToGame(parsed.data);
    }

    // TODO: Better error handling
    console.error(JSON.stringify(parsed.errors, null, 2));
    console.error("Error parsing selected game:", apiSelectedGame);
    return null;
  }

  // If no game is selected, return null
  return null;
}
