import { invoke } from "@tauri-apps/api";
import Database from "tauri-plugin-sql-api";
import { createTableWork, createTableTopic } from './init';
import type { NewTopic, NewWork } from "./types";
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

export function getTopic(id: string) {
  return globalDb.select("select * from topic where id = $1", [id]);
}

export async function deleteTopic(topicId: string) {
  await globalDb.execute("delete from work where topic_id=$1", [topicId]);
  await globalDb.execute("delete from topic where id=$1", [topicId]);
}

// event
export function getWorks(topicId: string) {
  return globalDb.select("select * from work where topic_id=$1", [topicId]);
}

export async function createWork(event: NewWork) {
  const newId = await invoke("get_uuid");

  return globalDb.execute(
    "insert into work (id, began_at, ended_at, desc, topic_id) values ($1, $2, $3, $4, $5)",
    [newId, event.began_at, event.ended_at, event.desc, event.topic_id]
  )
}
