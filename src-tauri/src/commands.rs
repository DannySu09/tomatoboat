
use uuid::Uuid;
use crate::action;
use std::path::Path;

#[tauri::command]
pub fn get_uuid() -> String {
  Uuid::new_v4().to_string()
}

#[tauri::command]
pub fn block_websites() {
  action::block_websites::block_websites(
    Path::new("/etc/hosts"),
    vec!["baidu.com"],
    "### by tomatoboat ###"
  );
}

#[tauri::command]
pub fn unblock_websites() {
  action::block_websites::unblock_websites(Path::new("/etc/hosts"), "### by tomatoboat ###");
}
