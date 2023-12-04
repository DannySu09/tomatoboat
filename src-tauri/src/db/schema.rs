diesel::table! {
  topic (id) {
    id -> Integer,
    title -> Varchar,
    desc -> VarChar,
    created_at -> Integer,
    modified_at -> Integer,
  }
}

diesel::table! {
    event (id) {
        id -> Integer,
        title -> Varchar,
        topic_id -> Integer,
        created_at -> Integer,
        began_at -> Integer,
        ended_at -> Integer,
    }
}

diesel::table! {
  work (id) {
    id -> Integer,
    event_id -> Integer,
    began_at -> Integer,
    ended_at -> Integer,
    desc -> VarChar,
  }
}