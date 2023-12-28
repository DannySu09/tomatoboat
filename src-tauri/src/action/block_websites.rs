use std::fs::{File, OpenOptions, read_to_string, remove_file, rename, remove_dir_all};
use std::io::Write;
use std::path::Path;
use home;

// write host mapping to the host file
pub fn block_websites(path: &Path, domain_list: Vec<&str>, mark: &str) {
  unblock_websites(path, mark);

  let mut file = match read_host_file(&path) {
    Ok(file) => file,
    Err(_) => {
      println!("Failed to block websites");
      return;
    },
  };

  let mut content_to_write: Vec<String> = Vec::new();

  for domain in domain_list.into_iter() {
    let has_www = domain.starts_with("www");

    if !has_www {
      content_to_write.push(format!("0.0.0.0 www.{}", domain));
      content_to_write.push(format!(":: www.{}", domain));
    }
    content_to_write.push(format!("0.0.0.0 {}", domain));
    content_to_write.push(format!(":: {}", domain));
  }

  if let Err(_) = writeln!(file, "{}", mark) {
    eprintln!("Write starting mark failed");
  }

  for host in content_to_write {
    if let Err(_) = writeln!(file, "{}", host) {
      eprintln!("Write line failed");
    }
  }

  if let Err(_) = writeln!(file, "{}", mark) {
    eprintln!("Write ending mark failed");
  }

  remove_browsers_cache();
}

pub fn unblock_websites(path: &Path, mark: &str) {
  let file = match read_to_string(path) {
    Ok(file) => file,
    Err(_) => return,
  };

  let lines = file.lines();

  let mut is_inside_modified_content = false;
  let bk_file_path_str = String::from(path.to_str().unwrap()) + ".bk";
  let bk_file_path = Path::new(&bk_file_path_str);
  let mut bk_file = read_host_file(&bk_file_path).unwrap();

  for line in lines  {
    if line == mark {
      is_inside_modified_content = !is_inside_modified_content;
      continue;
    }

    if is_inside_modified_content || line.len() == 0 {
      continue;
    }

    if let Err(_) = writeln!(bk_file, "{}", line) {
      // eprintln!("Recover Host file failed.");
    }
  }

  match remove_file(path) {
      Ok(_) => {
        let _ = rename(bk_file_path, path);
      }
      Err(_) => {
        eprintln!("Failed to remove Host file");
      }
  }
}

fn read_host_file(path: &Path) -> Result<File, std::io::Error> {
  OpenOptions::new().write(true).create(true).append(true).open(path)
}

// copied from https://github.com/SelfControlApp/selfcontrol/blob/master/Common/Utility/SCHelperToolUtilities.m
const BROWSERS_CACHE_PATH: [&str; 5] = [
  // chrome
  "/Library/Caches/Google/Chrome/Default",
  "/Library/Caches/Google/Chrome/com.google.Chrome",

  // firefox
  "/Library/Caches/Firefox/Profiles",

  // safari
  "/Library/Caches/com.apple.Safari",
  "/Library/Containers/com.apple.Safari/Data/Library/Caches"
];

fn remove_browsers_cache() {
  let home_dir_path = home::home_dir().unwrap();
  let home_dir = home_dir_path.to_str().unwrap();

  let browsers_cache_path_iter = BROWSERS_CACHE_PATH.iter()
    .map(|entry| {
      home_dir.to_string() + entry
    });

  for cache_path in browsers_cache_path_iter {
    remove_dir_all(Path::new(&cache_path)).unwrap_or_else(|e| {
      eprintln!("Failed to remove cache at {}: {}", cache_path, e.to_string())
    });
  }
}
