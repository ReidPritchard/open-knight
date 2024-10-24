
CREATE TABLE games (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  pgn TEXT NOT NULL,
  player_white TEXT,
  player_black TEXT,
  event TEXT,
  date TEXT,
  result TEXT,
  annotations TEXT,
  opening_name TEXT
);

CREATE TABLE moves (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  game_id INTEGER,
  move_number INTEGER,
  move_san TEXT,
  variation_id INTEGER DEFAULT 0,
  parent_variation_id INTEGER DEFAULT NULL,
  fen TEXT,
  annotation TEXT,
  FOREIGN KEY (game_id) REFERENCES games(id)
);
