import typia from "typia";
import type { APIGame } from "./bindings/APIGame";
import type { ExplorerGame } from "./bindings/ExplorerGame";

const parseAPIGame = typia.json.createValidateParse<APIGame>();

const parseExplorerState = typia.json.createValidateParse<{
  games: ExplorerGame[];
}>();

/**
 * Convert an api response "get_explorer_state" (JSON string) to an array of ExplorerGame
 */
export function apiExplorerStateToExplorerGames(
  apiExplorerState: string
): ExplorerGame[] {
  console.log("Parsing API Explorer State");

  const parsed = parseExplorerState(apiExplorerState);

  if (parsed.success) {
    return parsed.data.games;
  }

  console.error(
    "Error parsing explorer state:",
    JSON.stringify(parsed.errors, null, 2)
  );
  throw new Error(parsed.errors.join("\n"));
}

/**
 * Api response "get_selected_game" (JSON string) to a Game (as defined in src/App.vue)
 */
export function apiSelectedGameToGame(
  apiSelectedGame: string | null
): APIGame | null {
  console.log("Parsing API Selected Game");
  if (apiSelectedGame !== null) {
    const parsed = parseAPIGame(apiSelectedGame);
    if (parsed.success) {
      return parsed.data;
    }

    console.error(JSON.stringify(parsed.errors, null, 2));
    console.error("Error parsing selected game:", apiSelectedGame);
    return null;
  }

  return null;
}
