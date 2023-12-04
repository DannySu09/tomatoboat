-- Your SQL goes here
CREATE TABLE topic (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  desc TEXT,
  created_at DATE NOT NULL,
  modified_at DATE
);

CREATE TABLE event (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  began_at DATE,
  ended_at DATE,
  created_at DATE
);

CREATE TABLE work (
  id INTEGER PRIMARY KEY,
  event_id INTEGER NOT NULL,
  began_at INTEGER NOT NULL,
  ended_at INTEGER,
  desc TEXT
);
