// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ChessHeader } from "./ChessHeader";
import type { ChessMoveTree } from "./ChessMoveTree";
import type { ChessOpening } from "./ChessOpening";
import type { ChessPlayer } from "./ChessPlayer";
import type { ChessTournament } from "./ChessTournament";

export type ChessGame = { id: number, white_player: ChessPlayer, black_player: ChessPlayer, tournament: ChessTournament | null, opening: ChessOpening | null, result: string, round: number | null, date: string, headers: Array<ChessHeader>, move_tree: ChessMoveTree, fen: string | null, variant: string, pgn: string | null, tags: Array<string>, };
