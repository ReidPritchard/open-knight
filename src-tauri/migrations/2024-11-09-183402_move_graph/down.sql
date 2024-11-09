-- 1. Rename variation_order back to variation_id
ALTER TABLE moves RENAME COLUMN variation_order TO variation_id;

-- 2. Add the parent_variation_id column back to the moves table
ALTER TABLE moves ADD COLUMN parent_variation_id INTEGER;

-- 3. Add the fen column back to the moves table
ALTER TABLE moves ADD COLUMN fen TEXT NOT NULL;

-- 4. Add the annotation column back to the moves table
ALTER TABLE moves ADD COLUMN annotation TEXT DEFAULT NULL;

-- 5. Populate moves.fen and moves.annotation from the positions table
UPDATE moves
SET
    fen = (SELECT fen FROM positions WHERE positions.id = moves.child_position_id),
    annotation = (SELECT annotation FROM positions WHERE positions.id = moves.child_position_id);

-- 6. Remove the foreign key constraints from moves
ALTER TABLE moves DROP FOREIGN KEY (parent_position_id);
ALTER TABLE moves DROP FOREIGN KEY (child_position_id);

-- 7. Drop the parent_position_id and child_position_id columns from moves
ALTER TABLE moves DROP COLUMN parent_position_id;
ALTER TABLE moves DROP COLUMN child_position_id;

-- 8. Drop the positions table
DROP TABLE positions;
