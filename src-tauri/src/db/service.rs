use diesel::prelude::*;
use super::models::{Topic, NewTopic};

pub fn get_recent_topics(conn: &mut SqliteConnection) -> Vec<Topic> {
  use super::schema::topic::dsl::*;

  topic
    .limit(5)
    .order(created_at.desc())
    .load::<Topic>(conn)
    .expect("Error loading topics")
}

pub fn create_topic(conn: &mut SqliteConnection, title: &str, desc: &str) -> Topic {
  use super::schema::topic;

  let new_topic = NewTopic { title, desc };
  diesel::insert_into(topic::table)
    .values(&new_topic)
    .returning(Topic::as_returning())
    .get_result(conn)
    .expect("test")
}