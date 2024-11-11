-- Rename the date_text column back to date
ALTER TABLE games
RENAME COLUMN date_text TO date;
