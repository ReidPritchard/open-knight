
-- Create the Position table
CREATE TABLE positions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    fen TEXT NOT NULL,
    annotation TEXT DEFAULT NULL
    -- Moves will reference the position id
    -- as either the parent or child
);

-- Move will be the "edge" between two positions
-- we should add parent and child references to the move
ALTER TABLE moves ADD COLUMN parent_position_id INTEGER NOT NULL;
ALTER TABLE moves ADD COLUMN child_position_id INTEGER NOT NULL;

-- Add foreign key constraints
ALTER TABLE moves ADD FOREIGN KEY (parent_position_id) REFERENCES positions(id);
ALTER TABLE moves ADD FOREIGN KEY (child_position_id) REFERENCES positions(id);

-- Move the FENs in the moves table to the positions table
INSERT INTO positions (fen) SELECT fen FROM moves;

-- Update the moves table to reference the new positions table
UPDATE moves SET child_position_id = (SELECT id FROM positions WHERE moves.fen = positions.fen);
-- We can't set the parent_position_id as we don't know what it is
-- (we need to find it by referencing the previous move)

-- Drop the FEN column from the moves table
ALTER TABLE moves DROP COLUMN fen;
-- Drop parent_variation_id
ALTER TABLE moves DROP COLUMN parent_variation_id;

-- Rename the variation_id to variation_order
ALTER TABLE moves RENAME COLUMN variation_id TO variation_order;
