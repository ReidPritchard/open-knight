import { Chess } from "chess.js";
import typia, { tags } from "typia";


export interface IGame {
    id: string & tags.MinLength<1>;
    headers: Record<string, string>;
    game: Chess;
    pgn: string;
    errors: string[];
}
export const assertGame = typia.createAssert<IGame>();
export const validateGame = typia.createValidate<IGame>();

export interface IGameBoardGame extends Omit<IGame, 'game'> { }
export const assertGameBoardGame = typia.createAssert<IGameBoardGame>();
export const validateGameBoardGame = typia.createValidate<IGameBoardGame>();


export interface IExplorerState {
    games: IGame[];
}
export const assertExplorerState = typia.createAssert<IExplorerState>();
export const validateExplorerState = typia.createValidate<IExplorerState>();