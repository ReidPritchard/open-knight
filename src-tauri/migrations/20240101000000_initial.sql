PRAGMA foreign_keys = ON;
CREATE TABLE Player (
    player_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    elo_rating INTEGER,
    country TEXT,
    created_at TEXT,
    updated_at TEXT
);
CREATE TABLE Tournament (
    tournament_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    type TEXT,
    start_date TEXT,
    end_date TEXT,
    location TEXT
);
CREATE TABLE Opening (
    opening_id INTEGER PRIMARY KEY AUTOINCREMENT,
    eco_code TEXT,
    name TEXT,
    variation TEXT
);
CREATE TABLE Game (
    game_id INTEGER PRIMARY KEY AUTOINCREMENT,
    white_player_id INTEGER,
    black_player_id INTEGER,
    tournament_id INTEGER,
    opening_id INTEGER,
    result TEXT,
    round_number INTEGER,
    date_played TEXT,
    fen TEXT,
    pgn TEXT,
    created_at TEXT,
    FOREIGN KEY (white_player_id) REFERENCES Player(player_id),
    FOREIGN KEY (black_player_id) REFERENCES Player(player_id),
    FOREIGN KEY (tournament_id) REFERENCES Tournament(tournament_id),
    FOREIGN KEY (opening_id) REFERENCES Opening(opening_id)
);
CREATE TABLE Position (
    position_id INTEGER PRIMARY KEY AUTOINCREMENT,
    fen TEXT NOT NULL,
    fen_hash TEXT,
    created_at TEXT
);
CREATE TABLE Move (
    move_id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL,
    parent_move_id INTEGER,
    move_number INTEGER,
    player_color TEXT,
    move_notation TEXT,
    position_id INTEGER,
    created_at TEXT,
    FOREIGN KEY (game_id) REFERENCES Game(game_id),
    FOREIGN KEY (parent_move_id) REFERENCES Move(move_id),
    FOREIGN KEY (position_id) REFERENCES Position(position_id)
);
CREATE TABLE Annotation (
    annotation_id INTEGER PRIMARY KEY AUTOINCREMENT,
    move_id INTEGER NOT NULL,
    user_id INTEGER,
    comment TEXT,
    arrows TEXT,
    highlights TEXT,
    created_at TEXT,
    FOREIGN KEY (move_id) REFERENCES Move(move_id)
);
CREATE TABLE Evaluation (
    evaluation_id INTEGER PRIMARY KEY AUTOINCREMENT,
    position_id INTEGER NOT NULL,
    evaluation_score REAL,
    evaluation_type TEXT,
    depth INTEGER,
    engine_name TEXT,
    created_at TEXT,
    FOREIGN KEY (position_id) REFERENCES Position(position_id)
);
CREATE TABLE MoveTimeTracking (
    time_tracking_id INTEGER PRIMARY KEY AUTOINCREMENT,
    move_id INTEGER NOT NULL,
    time_spent_ms INTEGER,
    time_left_ms INTEGER,
    created_at TEXT,
    FOREIGN KEY (move_id) REFERENCES Move(move_id)
);
CREATE TABLE Tag (
    tag_id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT
);
CREATE TABLE GameTag (
    game_tag_id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (game_id) REFERENCES Game(game_id),
    FOREIGN KEY (tag_id) REFERENCES Tag(tag_id)
);
CREATE TABLE MoveTag (
    move_tag_id INTEGER PRIMARY KEY AUTOINCREMENT,
    move_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    FOREIGN KEY (move_id) REFERENCES Move(move_id),
    FOREIGN KEY (tag_id) REFERENCES Tag(tag_id)
);