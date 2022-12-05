use std::fs;
use std::process;
use regex::Regex;

fn reorder_crates(path_to_data : &str) -> String {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let mut stack_count = 0;

  let mut stacks:Vec<Vec<char>> = Vec::new();
  let re = Regex::new(r"(\d*)").unwrap();


  for row in contents.lines() {
    if stack_count == 0 {
      stack_count = row.len() / 4 + 1;

      for _a in 0..stack_count {
        stacks.push(Vec::new());
      }

    }

    if row.starts_with("move") {
      let matches:Vec<i32> = re.find_iter(row).filter_map(|d| d.as_str().parse().ok()).collect();

      let count = matches[0];
      let from:usize = matches[1].try_into().unwrap();
      let to:usize = matches[2].try_into().unwrap();

      for _i in 0..count {
        let item = stacks[from-1].pop().unwrap();
        stacks[to-1].push(item);
      }

    } else if !row.is_empty() {
      for i in 0..stack_count {
        let idx = 4*i + 1;
        let ch = row.chars().nth(idx).unwrap();
        if ch.is_ascii_alphabetic() {
          stacks[i].insert(0, ch);
        }
      }
    }
  }

 
  let mut res = String::new();

  for mut s in stacks {
    let item = s.pop().unwrap();
    res += &item.to_string();
  }

  return res;
}

fn reorder_crates_9001(path_to_data : &str) -> String {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let mut stack_count = 0;

  let mut stacks:Vec<Vec<char>> = Vec::new();
  let re = Regex::new(r"(\d*)").unwrap();


  for row in contents.lines() {
    if stack_count == 0 {
      stack_count = row.len() / 4 + 1;

      for _a in 0..stack_count {
        stacks.push(Vec::new());
      }

    }

    if row.starts_with("move") {
      let matches:Vec<i32> = re.find_iter(row).filter_map(|d| d.as_str().parse().ok()).collect();

      let count = matches[0];
      let from:usize = matches[1].try_into().unwrap();
      let to:usize = matches[2].try_into().unwrap();

      let mut tmp:Vec<char> = Vec::new();

      for _i in 0..count {
        let item = stacks[from-1].pop().unwrap();
        tmp.push(item);
      }

      for _i in 0..count {
        let item = tmp.pop().unwrap();
        stacks[to-1].push(item);
      }

    } else if !row.is_empty() {
      for i in 0..stack_count {
        let idx = 4*i + 1;
        let ch = row.chars().nth(idx).unwrap();
        if ch.is_ascii_alphabetic() {
          stacks[i].insert(0, ch);
        }
      }
    }
  }
  
  let mut res = String::new();

  for mut s in stacks {
    let item = s.pop().unwrap();
    res += &item.to_string();
  }

  return res;
}

fn main() {
  let order = reorder_crates("test.txt");
  assert!(order == "CMZ", "9000: The test tops is not correct!");

  let order = reorder_crates("data.txt");
  println!("9000: The top crates are (in order): {}", order);
  assert!(order == "WSFTMRHPP", "9000: The test tops is not correct!");

  let order = reorder_crates_9001("test.txt");
  assert!(order == "MCD", "9001: The test tops is not correct!");

  let order = reorder_crates_9001("data.txt");
  println!("9001: The top crates are (in order): {}", order);
  assert!(order == "GSLCMFBRP", "9001: The test tops is not correct!");
}
