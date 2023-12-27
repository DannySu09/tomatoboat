import Database from "tauri-plugin-sql-api";

export function createTableTopic(db: Database) {
  return db.execute(
    `
CREATE TABLE topic (
  id VARCHAR PRIMARY KEY,
  title TEXT NOT NULL,
  desc TEXT,
  created_at DATETIME NOT NULL,
  modified_at DATETIME
);
    `
  );
}

export function createTableWork(db: Database) {
  return db.execute(
    `
CREATE TABLE work (
  id VARCHAR PRIMARY KEY,
  topic_id VARCHAR NOT NULL,
  began_at DATETIME NOT NULL,
  ended_at DATETIME,
  desc TEXT
);
    `
  );
}

