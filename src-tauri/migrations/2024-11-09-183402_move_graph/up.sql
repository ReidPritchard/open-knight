-- We are already in a transaction
-- Step 1: Create the positions table with unique FENs
CREATE TABLE
positions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fen TEXT NOT NULL UNIQUE,
    annotation TEXT DEFAULT NULL
-- You can add more columns if needed
);

-- Step 2: Insert unique FENs into positions
INSERT INTO
positions (fen)
SELECT DISTINCT fen
FROM
    moves;

-- Step 3: Create the new_moves table with the updated schema
CREATE TABLE
new_moves (
    -- Existing columns
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER NOT NULL,
    move_number INTEGER NOT NULL,
    move_san TEXT NOT NULL,
    annotation TEXT,
    -- Updated/new columns
    variation_order INTEGER, -- Renamed from variation_id
    parent_position_id INTEGER, -- New column
    child_position_id INTEGER NOT NULL, -- New column
    -- Foreign key constraints
    FOREIGN KEY (parent_position_id) REFERENCES positions (id),
    FOREIGN KEY (child_position_id) REFERENCES positions (id)
);

-- Step 4: Populate new_moves with data from moves, linking to positions
INSERT INTO
new_moves (
    id,
    game_id,
    move_number,
    move_san,
    annotation,
    variation_order,
    parent_position_id,
    child_position_id
)
SELECT
    m.id,
    m.game_id,
    m.move_number,
    m.move_san,
    m.annotation,
    m.variation_id AS variation_order,
    NULL AS parent_position_id, -- Unable to set at this time
    p.id AS child_position_id
FROM
    moves AS m
INNER JOIN positions AS p ON m.fen = p.fen;

-- Step 5: Drop the old moves table
DROP TABLE moves;

-- Step 6: Rename new_moves to moves
ALTER TABLE new_moves
RENAME TO moves;

-- Step 7: Recreate indexes
CREATE INDEX idx_moves_game_id ON moves (game_id);
