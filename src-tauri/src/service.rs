use diesel::prelude::*;
use crate::models::{Topic, NewTopic};

pub fn get_all_topics(conn: &mut SqliteConnection) -> Vec<Topic> {
  use crate::schema::topic::dsl::*;

  topic.limit(5).load::<Topic>(conn).expect("Error loading topics")
}

pub fn create_topic(conn: &mut SqliteConnection, title: &str, desc: &str) -> Topic {
  use crate::schema::topic;

  let new_topic = NewTopic { title, desc };
  diesel::insert_into(topic::table)
    .values(&new_topic)
    .returning(Topic::as_returning())
    .get_result(conn)
    .expect("test")
}