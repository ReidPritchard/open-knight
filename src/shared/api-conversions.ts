import typia from "typia";
import {
  type IAPIGame,
  type IAPIMove,
  type IExplorerGame,
  type IGame,
  assertAPIGame,
} from "./types";

/**
 * This is an intermediate type that matches the raw json response from the tauri api
 * It should not be used outside of this file as it's difficult to work with
 */
interface PartialExplorerState {
  games: {
    id: number | null;
    event: string | null;
    date_text: string | null;
    result: string | null;
    player_white: string | null;
    player_black: string | null;
    opening_name: string | null;
    annotations: string | null;
    pgn: string;
    moves: IAPIMove[];
    errors: string[];
  }[];
}
const parsePartialExplorerState = typia.json.createValidateParse<PartialExplorerState>();
const parsePartialGame = typia.json.createValidateParse<PartialExplorerState["games"][number]>();

/**
 * Convert an incoming game from the API to an IAPIGame
 */
export const gameResultToGame = (
  game: PartialExplorerState["games"][number]
): IAPIGame => {
  console.log("Converting game result to api game:", game.id);

  const apiGame = assertAPIGame({
    game_data: {
      id: game.id,
      event: game.event,
      date_text: game.date_text,
      result: game.result,
      player_white: game.player_white,
      player_black: game.player_black,
      opening_name: game.opening_name,
      annotations: game.annotations,
      pgn: game.pgn,
      errors: game.errors,
    },
    moves: game.moves,
  });

  console.log("Api game:", apiGame);
  return apiGame;
};

/**
 * Convert an api response "get_explorer_state" (JSON string) to an array of IExplorerGame
 */
export function apiExplorerStateToExplorerGames(
  apiExplorerState: string
): IExplorerGame[] {
  console.log("Parsing API Explorer State");

  const parsed = parsePartialExplorerState(apiExplorerState);

  if (parsed.success) {
    return parsed.data.games
      .filter((game) => game.id !== null)
      .map((game) => ({
        id: game.id as number,
        headers: [
          ["Event", game.event ?? ""],
          ["Date", game.date_text ?? ""],
          ["Result", game.result ?? ""],
          ["White", game.player_white ?? ""],
          ["Black", game.player_black ?? ""],
          ["Opening", game.opening_name ?? ""],
          ["Annotations", game.annotations ?? ""],
        ],
      }));
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
): IAPIGame | null {
  console.log("Parsing API Selected Game");
  if (apiSelectedGame !== null) {
    const parsed = parsePartialGame(apiSelectedGame);
    if (parsed.success) {
      return gameResultToGame(parsed.data);
    }

    console.error(JSON.stringify(parsed.errors, null, 2));
    console.error("Error parsing selected game:", apiSelectedGame);
    return null;
  }

  return null;
}
