use std::fs;
use std::process;

fn get_duplicate_char_in_bag(bag_string:&str) -> char {
  let mut duplicate = ' ';
  let (s1, s2) = bag_string.split_at(bag_string.len()/2);

  for c1 in s1.chars() {
    for c2 in s2.chars() {
      if c1 == c2 {
        duplicate = c1;
      }
    }
  }
  return duplicate;
}

fn score_of_item(item: char) -> u32 {
  let idx = item as u32;
  let score;

  if item.is_ascii_uppercase() {
    score = idx - 65 + 27;
  } else {
    score = idx - 97 + 1;
  }

  return score;
}

fn sum_priorities(path_to_data : &str) -> u32 {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let split = contents.split("\n");
  let mut total_points = 0;
  for bag_string in split {
    let duplicate = get_duplicate_char_in_bag(bag_string);
    let score = score_of_item(duplicate);
    total_points += score;
  }

  return total_points;

}

fn sum_badge_priorities(path_to_data : &str) -> u32 {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let elf_bags: Vec<&str> = contents.split("\n").collect();
  let group_bags: Vec<&[&str]> = elf_bags.chunks(3).collect();
  let mut total_points = 0;

  // Each group
  for g in group_bags {
    let mut group_badge = ' ';
    // Iterate individuals
    for c1 in g[0].chars() {
      for c2 in g[1].chars() {
        for c3 in g[2].chars() {
          if c1 == c2 && c2 == c3 {
            group_badge = c1;
          }
        }
      }
    }

    total_points += score_of_item(group_badge);

  }

  return total_points;

}

fn main() {
  let score = sum_priorities("test.txt");
  assert!(score == 157, "The test sum is not correct!");

  let score = sum_priorities("data.txt");
  println!("The sum of individual items are: {}", score);
  assert!(score == 7727, "The data sum is not correct!");

  let score = sum_badge_priorities("test.txt");
  assert!(score == 70, "The test sum for badges is not correct!");

  let score = sum_badge_priorities("data.txt");
  println!("Your sum of badges are: {}", score);
  assert!(score == 2609, "The data sum for badges is not correct!");
}
