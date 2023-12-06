// @generated automatically by Diesel CLI.

diesel::table! {
    event (id) {
        id -> Integer,
        title -> Text,
        began_at -> Timestamp,
        ended_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    topic (id) {
        id -> Integer,
        title -> Text,
        desc -> Nullable<Text>,
        created_at -> Timestamp,
        modified_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    work (id) {
        id -> Integer,
        event_id -> Integer,
        began_at -> Timestamp,
        ended_at -> Nullable<Timestamp>,
        desc -> Nullable<Text>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    event,
    topic,
    work,
);
