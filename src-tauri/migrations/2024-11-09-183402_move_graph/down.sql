-- Step 1: Create the old_moves table with the original schema
CREATE TABLE
old_moves (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    game_id INTEGER,
    fen TEXT NOT NULL DEFAULT '',
    move_number INTEGER,
    variation_id INTEGER,
    parent_variation_id INTEGER,
    move_san TEXT,
    annotation TEXT
);

-- Step 2: Populate old_moves with data from moves, linking back to FENs
INSERT INTO
old_moves (
    id,
    game_id,
    fen,
    move_number,
    variation_id,
    parent_variation_id,
    move_san,
    annotation
)
SELECT
    m.id,
    m.game_id,
    p.fen,
    m.move_number,
    m.variation_order AS variation_id,
    NULL AS parent_variation_id,
    m.move_san,
    m.annotation
FROM
    moves AS m
INNER JOIN positions AS p ON m.child_position_id = p.id;

-- Step 3: Drop the new moves table
DROP TABLE moves;

-- Step 4: Rename old_moves back to moves
ALTER TABLE old_moves
RENAME TO moves;

-- Step 5: Drop the positions table
DROP TABLE positions;

-- Step 6: Recreate indexes
-- CREATE INDEX idx_moves_game_id ON moves (game_id);
-- There were no indexes on the moves table to begin with
