use std::fs;
use std::process;
use std::collections::HashMap;

fn get_dir_map(path_to_data : &str) -> HashMap<String, u64> {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let mut dirs = HashMap::new();
  let mut stack:Vec<&str> = Vec::new();

  for line in contents.lines() {
    let mut parts = line.split(" ");
    match (parts.next(), parts.next(), parts.next()) {
      (Some("$"), Some("cd"), Some("..")) => {
        stack.pop();
      },
      (Some("$"), Some("cd"), Some(dir)) => {
        // Add the new dir to the stack and to the dir map
        stack.push(dir);
        dirs.insert(stack.join(""), 0);        
      },
      (Some("$"), Some(_), None) => {
        // Do not care about value, but match this
      },
      (Some("dir"), Some(_), None) => {
        // Do not care about value, but match this
      },
      (Some(size_string), Some(_), None) => {
        let size:u64 = size_string.parse().unwrap();

        let mut path = String::new();

        for l in &stack {
          path.push_str(l);
          dirs.insert(path.clone(), size + dirs[&path]);
        }
      },
      (_,_,_) => unreachable!(),
    }
  }

  return dirs;
}

fn calculate_size_of_files(path_to_data : &str) -> u64 {
  let dirs = get_dir_map(path_to_data);

  let size = dirs.into_iter().filter(|(_, v)| *v <= 100000).map(|(_, v)| v).sum();

  return size;
}

fn calculate_size_to_delete(path_to_data : &str) -> u64 {
  let dirs = get_dir_map(path_to_data);

  let min_delete_size = 30000000 - (70000000 - dirs["/"]);

  let mut smallest_size = 70000000;
  let mut smallest_path = String::new();

  for (k, v) in &dirs {
    if v < &smallest_size && v >= &min_delete_size {
      smallest_size = *v;
      smallest_path = k.clone();
    }
  }

  return dirs[&smallest_path];
}

fn main() {
  let size = calculate_size_of_files("test.txt");
  assert!(size == 95437, "The test dir size is not correct!");

  let size = calculate_size_of_files("data.txt");
  println!("Sum of all dirs below 100k: {}", size);
  assert!(size == 1315285, "The data dir size is not correct!");

  let size = calculate_size_to_delete("test.txt");
  assert!(size == 24933642, "The test dir size is not correct!");

  let size = calculate_size_to_delete("data.txt");
  println!("Size of directory to delete: {}", size);
  assert!(size == 9847279, "The data dir size is not correct!");
}
