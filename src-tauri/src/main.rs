// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn get_topics() -> String {
    let mut conn = db::connect::conn_db();
    let topics = db::service::get_recent_topics(&mut conn);

    serde_json::to_string(&topics).expect("Failed to getting json string of recent topics")
}

#[tauri::command]
fn create_topic(msg: String) -> String {
    let mut conn = db::connect::conn_db();
    println!("{}", &msg);
    let payload = serde_json::from_str::<db::models::NewTopic>(&msg).expect("Failed to parse json into Payload");
    let new_topic = db::service::create_topic(&mut conn, payload.title, payload.desc);
    let new_topic_str = serde_json::to_string(&new_topic).expect("Failed to creating json string");

    new_topic_str
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            db::connect::run_migrations();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![get_topics, create_topic])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
