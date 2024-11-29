import typia from "typia";
import type { ExplorerGame } from "./bindings/ExplorerGame";

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
