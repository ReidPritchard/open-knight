// This file was automatically generated by ts_bind, do not modify it manually
import type { Position } from "./Position";
import type { Move } from "./Move";

export interface APIMove {
   game_move: Move;
   parent_position: Position;
   child_position: Position;
}