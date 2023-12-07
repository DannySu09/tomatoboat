import { invoke } from "@tauri-apps/api";
import Database from "tauri-plugin-sql-api";
import { createTableEvent, createTableWork, createTableTopic } from './init';
import type { NewTopic } from "./types";

let globalDb: Database;

export async function init(cb: () => void) {
  let db: Database;
  if (globalDb) {
    db = globalDb;
  } else {
    globalDb = db = await Database.load("sqlite:tomatoboat.db");
  }

  try {
    await createTableEvent(db);
    await createTableWork(db);
    await createTableTopic(db);
  } catch(e) {} finally {
    cb();
  }
}

export function getRecentTopics() {
  return globalDb.select(
    "select * from topic"
  );
}

export async function createTopic(topic: NewTopic) {
  const newId = await invoke("get_uuid");
  console.log(newId);
  return globalDb.execute("insert into topic (id, title, desc, created_at) values ($1, $2, $3, $4)", [newId, topic.title, topic.desc, (new Date()).toISOString()]);
}
