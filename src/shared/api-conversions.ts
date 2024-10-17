import { Chess } from "chess.js";
import { assertExplorerState, assertGameBoardGame, assertGame, IExplorerState, IGameBoardGame, IGame } from "./types";


/**
 * This is an intermediate type that matches the raw json response from the tauri api
 * It should not be used outside of this file as it's difficult to work with
 */
type PartialExplorerState = {
    games: {
        id: string;
        // ex. [["Event", "Casual Game"], ["Result", "1-0"], ["White", "Thanos"], ["Black", "Thanos"]]
        headers: [string, string][];
        pgn: string;
        errors: string[];
    }[];
}

/**
 * Convert an incoming "GameResult" (as defined in src-tauri/src/state.rs) to a "Game" (as defined in src/App.vue)
 */
export const gameResultToGame = (game: PartialExplorerState['games'][number]): IGame => {
    const loaded_game = new Chess();
    console.log("Game pgn:", game.pgn);
    loaded_game.loadPgn(game.pgn);


    const parsed_headers: Record<string, string> = {};
    const headers = game.headers;
    for (const header_index in headers) {
        // header is ["key", "value"]
        const [key, value] = headers[header_index];
        parsed_headers[key.toLowerCase()] = value;
    }
    const pgn = loaded_game.pgn();

    return assertGame({
        id: game.id,
        headers: parsed_headers,
        game: loaded_game,
        pgn: pgn,
        errors: game.errors,
    });
}

/**
 * Convert an api response "get_explorer_state" (JSON string) to an ExplorerState (as defined in src/App.vue)
 */
export function apiExplorerStateToExplorerState(apiExplorerState: string): IExplorerState {
    const parsed = JSON.parse(apiExplorerState);
    return assertExplorerState(parsed);
}

/**
 * Api response "get_selected_game" (JSON string) to a Game (as defined in src/App.vue)
 */
export function apiSelectedGameToGame(apiSelectedGame: string): IGameBoardGame {
    const parsed = JSON.parse(apiSelectedGame);
    return assertGameBoardGame(gameResultToGame(parsed));
}

