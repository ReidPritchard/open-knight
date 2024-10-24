import typia, { tags } from "typia";

export interface IGame {
  id: string & tags.MinLength<1>;
  headers: Record<string, string>;
  pgn: string;
  errors: string[];
}
export const assertGame = typia.createAssert<IGame>();
export const validateGame = typia.createValidate<IGame>();

export interface IExplorerState {
  games: IGame[];
}
export const assertExplorerState = typia.createAssert<IExplorerState>();
export const validateExplorerState = typia.createValidate<IExplorerState>();
