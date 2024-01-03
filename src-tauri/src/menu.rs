use tauri::{Menu, CustomMenuItem, Submenu};

pub fn set_menus() -> Menu {
  let menu = Menu::new();

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let submenu = Submenu::new("File", Menu::new().add_item(quit));

  menu
    .add_submenu(submenu)
}

pub fn set_system_tray() -> SystemTray {}
