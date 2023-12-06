-- Your SQL goes here

CREATE TABLE work (
  id INTEGER PRIMARY KEY NOT NULL,
  event_id INTEGER NOT NULL,
  began_at DATETIME NOT NULL,
  ended_at DATETIME,
  desc TEXT
)
