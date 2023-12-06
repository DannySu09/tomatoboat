-- Your SQL goes here
CREATE TABLE topic (
  id INTEGER PRIMARY KEY NOT NULL,
  title TEXT NOT NULL,
  desc TEXT,
  created_at DATETIME NOT NULL,
  modified_at DATETIME
)
