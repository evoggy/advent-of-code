use std::fs;
use std::process;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn boarders(&self, other: &Self) -> bool {
    return (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1;
  }

  fn follow(&mut self, head: &Self) {

    if self.x > head.x {
      self.x -= 1;
    } else if self.x < head.x{
      self.x += 1;
    }
   
    if self.y > head.y {
      self.y -= 1;
    } else if self.y < head.y {
      self.y += 1;
    }
  }
}

#[derive(Debug)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug)]
struct Move {
  direction: Direction,
  distance: i32,
}

impl Move {
  fn new(move_line: &str) -> Move {
    let mut split = move_line.split(" ");
    return Move {
      direction: match split.next().unwrap() {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Invalid direction"),
      },
      distance: split.next().unwrap().parse().unwrap(),
    };
  }
}

fn calculate_tail_visits(path_to_data : &str, nbr_of_knots: usize) -> u32 {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let moves:Vec<Move> = contents.lines().map(|line| Move::new(line)).collect();

  let mut visited:Vec<Position> = Vec::new();

  let mut knots:Vec<Position> = Vec::new();
  for _i in 0..nbr_of_knots {
    knots.push(Position { x: 0, y: 0 });
  }

  visited.push(Position { x: 0, y: 0 });

  for m in moves.iter() { 
    for _ in 0..m.distance {
      match &m.direction {
        Direction::Up => knots[0].y += 1,
        Direction::Down => knots[0].y -= 1,
        Direction::Right => knots[0].x += 1,
        Direction::Left => knots[0].x -= 1,
      }

      for i in 0..knots.len() {
        // Do not move head here
        if i > 0 {
          let previous = knots[i-1];
          if !knots[i].boarders(&previous) {
            knots[i].follow(&previous);
          }
        }
        
        if !visited.contains(&knots[i]) && i == nbr_of_knots - 1 {
          visited.push(knots[i].clone());
        }
      } 
    }
  }

  return visited.len() as u32;
}


fn main() {
  let pos = calculate_tail_visits("test.txt", 2);
  assert!(pos == 13, "The number of positions the tail visited is not correct");

  let pos = calculate_tail_visits("data.txt", 2);
  println!("The number of positions the tail visited is: {}", pos);
  assert!(pos == 6376, "The number of positions the tail visited is not correct");
  
  let pos = calculate_tail_visits("test.txt", 10);
  assert!(pos == 1, "Short tail wrong");

  let pos = calculate_tail_visits("test-large.txt", 10);
  assert!(pos == 36, "Large tail wrong");

  let pos = calculate_tail_visits("data.txt", 10);
  println!("The number of positions the tail visited is: {}", pos);
  assert!(pos == 2607, "Real tail wrong");
}
