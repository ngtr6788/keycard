-- Your SQL goes here
CREATE TABLE decks (
  id INTEGER PRIMARY KEY NOT NULL,
  deck_name TEXT UNIQUE NOT NULL,
  deck_description TEXT
);
