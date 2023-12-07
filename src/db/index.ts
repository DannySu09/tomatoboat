import { invoke } from "@tauri-apps/api";
import Database from "tauri-plugin-sql-api";
import { createTableEvent, createTableWork, createTableTopic } from './init';
import type { NewTopic } from "./types";
import { validateNewTopic } from "../utils/validators";

let globalDb: Database;

export async function initDb(cb: () => void) {
  let db: Database;
  if (globalDb) {
    db = globalDb;
  } else {
    globalDb = db = await Database.load("sqlite:tomatoboat.db");
  }

  try {
    await createTableEvent(db);
  } catch(e) {}
  try {
    await createTableWork(db);
  } catch(e) {}
  try {
    await createTableTopic(db);
  } catch(e) {}

  cb();
}

export function getRecentTopics() {
  return globalDb.select(
    "select * from topic"
  );
}

export async function createTopic(topic: NewTopic) {
  validateNewTopic(topic);

  const newId = await invoke("get_uuid");
  return globalDb.execute("insert into topic (id, title, desc, created_at) values ($1, $2, $3, $4)", [newId, topic.title, topic.desc, (new Date()).toISOString()]);
}
