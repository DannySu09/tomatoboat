use std::fs;
use directories::ProjectDirs;
use diesel::prelude::*;

pub fn conn_db() -> SqliteConnection {
  let proj_dir = ProjectDirs::from("com", "naiwensu", "tomatoboat").unwrap();
  let proj_data_dir = proj_dir.data_dir();

  if proj_data_dir.exists() == false {
    fs::create_dir(&proj_data_dir).expect("Failed to create data directory");
  }

  if proj_data_dir.is_file() {
    panic!("A file with the same name has already existed!")
  }

  let proj_db_path = proj_data_dir.join("db.sqlite");
  let proj_db_path_str = proj_db_path.to_str().unwrap();

  SqliteConnection::establish(proj_db_path_str)
    .unwrap_or_else(|_e| {
      panic!("Error connecting to {}", proj_db_path_str)
    })
}

