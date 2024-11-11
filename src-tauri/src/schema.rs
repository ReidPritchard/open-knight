// @generated automatically by Diesel CLI.

diesel::table! {
    games (id) {
        id -> Integer,
        pgn -> Text,
        player_white -> Nullable<Text>,
        player_black -> Nullable<Text>,
        event -> Nullable<Text>,
        date_text -> Nullable<Text>,
        result -> Nullable<Text>,
        annotations -> Nullable<Text>,
        opening_name -> Nullable<Text>,
    }
}

diesel::table! {
    moves (id) {
        id -> Integer,
        game_id -> Integer,
        move_number -> Integer,
        move_san -> Text,
        annotation -> Nullable<Text>,
        variation_order -> Nullable<Integer>,
        parent_position_id -> Nullable<Integer>,
        child_position_id -> Integer,
    }
}

diesel::table! {
    positions (id) {
        id -> Integer,
        fen -> Text,
        annotation -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(games, moves, positions);
