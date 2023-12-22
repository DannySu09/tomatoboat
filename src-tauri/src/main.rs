// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod action;

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .plugin(tauri_plugin_sql::Builder::default().build())
        .invoke_handler(tauri::generate_handler![
            commands::get_uuid,
            commands::block_websites,
            commands::unblock_websites
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
