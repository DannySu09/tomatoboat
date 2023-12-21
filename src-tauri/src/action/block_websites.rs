use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;

pub fn block_websites(path: &Path, domain_list: Vec<&str>, mark: &str) {
  let mut file = read_host_file(&path).unwrap();
  let content_to_write = domain_list
    .into_iter()
    .map(|domain| {
      format!("{} {}", domain, "127.0.0.1")
    });

  let _ =file.write(mark.as_bytes());

  for host in content_to_write {
    let _ = file.write(host.as_bytes());
  }

  let _ = file.write(mark.as_bytes());
}

pub fn unblock_websites(path: &Path, mark: &str) {
  let file = read_host_file(&path).unwrap();

  let lines = BufReader::new(&file).lines();
  let mut is_inside_modified_content = false;
  let mut contents = Vec::new();

  for line in lines  {
    let line_content = line.unwrap();
    if &line_content == mark {
      is_inside_modified_content = !is_inside_modified_content;
    }

    if is_inside_modified_content {
      continue;
    }

    contents.push(line_content);
  }

  let bk_file_path = String::from(path.to_str().unwrap()) + ".bk";
  let mut new_file = File::create(&bk_file_path).unwrap();

  let _ = new_file.write(contents.join("\n").as_bytes());
  let _ = fs::remove_file(path);
  let _ = fs::rename(bk_file_path, path);
}

fn read_host_file(path: &Path) -> Result<File, &'static str> {
  if !path.exists() || !path.is_file() {
    return Err("File does not exist or is not a regular file")
  }

  let file = File::open(path);

  if file.is_err() {
    return Err("Could not open the file");
  }

  Ok(file.unwrap())
}
