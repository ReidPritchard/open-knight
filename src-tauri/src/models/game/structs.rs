use crate::models::ChessMoveTree;
use ok_utils::ts_export;
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
        pub headers: Vec<ChessHeader>,
        pub move_tree: ChessMoveTree,
        pub fen: Option<String>,
        pub variant: String,
        pub pgn: Option<String>,
        pub tags: Vec<String>,
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
    #[derive(Default)]
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
    #[derive(Default)]
    pub struct ChessOpening {
        pub id: i32,
        pub eco: Option<String>,
        pub name: Option<String>,
        pub variation: Option<String>,
    }
}

ts_export! {
    #[derive(Default)]
    pub struct ChessHeader {
        pub id: Option<i32>,
        pub game_id: i32,
        pub name: String,
        pub value: String,
    }
}
