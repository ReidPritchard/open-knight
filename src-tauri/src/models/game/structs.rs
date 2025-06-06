use crate::models::ChessMoveTree;
use crate::ts_export;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

ts_export! {
    pub struct ChessGame {
        pub id: i32,
        pub white_player: ChessPlayer,
        pub black_player: ChessPlayer,
        pub tournament: Option<ChessTournament>,
        pub opening: Option<ChessOpening>,
        pub result: String,
        pub round: Option<i32>,
        pub date: String,
        pub move_tree: ChessMoveTree,
        pub tags: Vec<String>,
        pub fen: Option<String>,
        pub pgn: Option<String>,
    }
}

ts_export! {
    pub struct ChessPlayer {
        pub id: i32,
        pub name: String,
        pub elo: Option<i32>,
        pub country: Option<String>,
    }
}

ts_export! {
    pub struct ChessTournament {
        pub id: i32,
        pub name: String,
        pub tournament_type: Option<String>,
        pub time_control: Option<String>,
        pub start_date: Option<String>,
        pub end_date: Option<String>,
        pub location: Option<String>,
    }
}

ts_export! {
    pub struct ChessOpening {
        pub id: i32,
        pub eco: Option<String>,
        pub name: Option<String>,
        pub variation: Option<String>,
    }
}
