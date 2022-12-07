use std::fs;
use std::process;
use regex::Regex;

struct Dir {
  name: String,
  size: usize
}

use std::collections::HashMap;

fn calculate_size_of_files(path_to_data : &str) -> u64 {
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
        if dirs.contains_key(dir) {
          println!("{} already exists", dir);
          panic!();
        }

        stack.push(dir);
        let path = stack.join("");

        dirs.insert(path, 0);
        
      },
      (Some("$"), Some(cmd), None) => {
        println!("$ {}", cmd);
      },
      (Some("dir"), Some(_), None) => {
        // Do not care
      },
      (Some(size_string), Some(_), None) => {
        let size:u64 = size_string.parse().unwrap();

        let mut path = String::new();

        for l in &stack {
          path.push_str(l);
          println!("{} gets {}", path,size);
          let path2 = path.clone();
          let path3 = path.clone();
          let prev = dirs.get(&path2).unwrap();
          dirs.insert(path3, size + prev);
        }
      },
      (_,_,_) => println!("Unknown command"),
    }
  }

  println!("---------------");

  let mut s:u64 = 0;

  for (k, v) in &dirs {
    println!("{}: {}", k, v);
    if v <= &100000 {
      s += v;
    }
  }

  dbg!(s);

  return s;
}


fn calculate_size_to_delete(path_to_data : &str) -> i64 {
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
        if dirs.contains_key(dir) {
          println!("{} already exists", dir);
          panic!();
        }

        stack.push(dir);
        let path = stack.join("");

        dirs.insert(path, 0);
        
      },
      (Some("$"), Some(cmd), None) => {
        println!("$ {}", cmd);
      },
      (Some("dir"), Some(_), None) => {
        // Do not care
      },
      (Some(size_string), Some(_), None) => {
        let size:i64 = size_string.parse().unwrap();

        let mut path = String::new();

        for l in &stack {
          path.push_str(l);
          println!("{} gets {}", path,size);
          let path2 = path.clone();
          let path3 = path.clone();
          let prev = dirs.get(&path2).unwrap();
          dirs.insert(path3, size + prev);
        }
      },
      (_,_,_) => println!("Unknown command"),
    }
  }

  println!("---------------");

  let size_to_free:i64 = 70000000 - dirs["/"];

  println!("Size to free: {}", size_to_free);

  let mut smallest_size = 70000000;
  let mut smallest_path = String::new();

  for (k, v) in &dirs {
    println!("{}: {}", k, v);
    if v < &smallest_size && v >= &size_to_free {
      smallest_size = *v;
      //println!("{}", smallest_size);
      smallest_path = k.clone();
    }
  }
  
  let node_size = dirs[&smallest_path];

  println!("{}::{}", smallest_path, node_size);



  return node_size;
}

fn main() {
  let order = calculate_size_of_files("test.txt");
  assert!(order == 95437, "9000: The test tops is not correct!");

  let order = calculate_size_of_files("data.txt");
  println!("9000: The top crates are (in order): {}", order);
  //assert!(order == "WSFTMRHPP", "9000: The test tops is not correct!");

  let order = calculate_size_to_delete("test.txt");
  assert!(order == 24933642, "9001: The test tops is not correct!");

  let order = calculate_size_to_delete("data.txt");
  println!("9001: The top crates are (in order): {}", order);
  //assert!(order == "GSLCMFBRP", "9001: The test tops is not correct!");
}
