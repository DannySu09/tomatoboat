use super::schema;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[derive(Clone, serde::Serialize)]
#[diesel(table_name = schema::topic)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Topic {
    pub id: i32,
    pub title: String,
    pub desc: String,
    pub created_at: i32,
    pub modified_at: i32,
}

#[derive(Insertable)]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = schema::topic)]
pub struct NewTopic<'a> {
  pub title: &'a str,
  pub desc: &'a str,
}

#[derive(Queryable, Selectable)]
#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = schema::event)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Event {
  pub id: i32,
  pub title: String,
  pub topic_id: i32,
  pub began_at: i32,
  pub ended_at: i32,
}

#[derive(Queryable, Selectable)]
#[derive(Clone, serde::Serialize)]
#[diesel(table_name = super::schema::work)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Work {
  pub id: i32,
  pub event_id: i32,
  pub began_at: i32,
  pub ended_at: i32,
  pub desc: String
}
