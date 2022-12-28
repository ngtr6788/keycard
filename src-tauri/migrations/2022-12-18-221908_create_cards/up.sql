-- Your SQL goes here
CREATE TABLE cards (
  deck_id INTEGER NOT NULL,
  id INTEGER NOT NULL PRIMARY KEY,
  card_question TEXT UNIQUE NOT NULL,
  keys_list TEXT NOT NULL,
  successful_reviews INTEGER NOT NULL,
  due_datetime TEXT NOT NULL, -- will be saved as ISO string
  efactor REAL NOT NULL
);
