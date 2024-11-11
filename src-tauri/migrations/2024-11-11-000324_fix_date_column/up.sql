-- Rename the date column to date_text so it's not using a keyword
ALTER TABLE games
RENAME COLUMN date TO date_text;
