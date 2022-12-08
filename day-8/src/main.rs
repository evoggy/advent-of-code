use std::fs;
use std::process;

fn create_tree_matrix(path_to_data : &str) -> Vec<Vec<u32>> {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let trees:Vec<Vec<u32>> = contents.lines().map(|line| {
    line.chars().map(|c| {
      return c.to_digit(10).unwrap();
    }).collect()
  }).collect();

  return trees;
}

fn calculate_visible_trees(path_to_data : &str) -> u32 {
  let trees = create_tree_matrix(path_to_data);

  let mut visible_trees = 0;

  for y in 0..trees.len() {
    for x in 0..trees[y].len() {
      if x == 0 || y == 0 || x == trees[y].len() - 1 || y == trees.len() - 1 {
        visible_trees += 1;
      } else {

        let mut is_visible_x_1 = true;
        let mut is_visible_x_2 = true;
        let mut is_visible_y_1 = true;
        let mut is_visible_y_2 = true;
        let this_tree = trees[y][x];
        
        for i in 0..x {
          if trees[y][i] >= this_tree {
            is_visible_x_1 = false;
            break;
          }
        }

        for i in x+1..trees[x].len() {
          if trees[y][i] >= this_tree {
            is_visible_x_2 = false;
            break;
          }
        }

        for i in 0..y {
          if trees[i][x] >= this_tree {
            is_visible_y_1 = false;
            break;
          }
        }

        for i in y+1..trees.len() {
          if trees[i][x] >= this_tree {
            is_visible_y_2 = false;
            break;
          }
        }

        if is_visible_x_1 || is_visible_x_2 || is_visible_y_1 || is_visible_y_2 {
          visible_trees += 1;
        }
      }
    }
  }

  return visible_trees;
}

fn calculate_max_scenic_score(path_to_data : &str) -> u32 {
  let trees = create_tree_matrix(path_to_data);

  let mut max_score = 0;

  for y in 0..trees.len() {
    for x in 0..trees[y].len() {

      let mut visable_x_1 = 0;
      let mut visible_x_2 = 0;
      let mut visible_y_1 = 0;
      let mut visible_y_2 = 0;

      let this_tree = trees[y][x];
      
      for i in (0..x).rev() {
        visable_x_1 += 1;
        if trees[y][i] >= this_tree {
          break;
        }
      }

      for i in x+1..trees[x].len() {
        visible_x_2 += 1;
        if trees[y][i] >= this_tree {
          break;
        }
      }

      for i in (0..y).rev() {
        visible_y_1 += 1;
        if trees[i][x] >= this_tree {
          break;
        }
      }

      for i in y+1..trees.len() {
        visible_y_2 += 1;
        if trees[i][x] >= this_tree {
          break;
        }
      }

      let this_tree_score = visable_x_1 * visible_x_2 * visible_y_1 * visible_y_2;

      if this_tree_score > max_score {
        max_score = this_tree_score;
      }
    }
  }

  return max_score;
}


fn main() {
  let size = calculate_visible_trees("test.txt");
  assert!(size == 21, "The test number of trees is not correct!");

  let size = calculate_visible_trees("data.txt");
  println!("The number of trees you can see is: {}", size);
  assert!(size == 1823, "The data dir size is not correct!");

  let size = calculate_max_scenic_score("test.txt");
  assert!(size == 8, "The test scenic score is not correct!");

  let size = calculate_max_scenic_score("data.txt");
  println!("Max scenic score is: {}", size);
  assert!(size == 211680, "The data scenic score is not correct!");
}
