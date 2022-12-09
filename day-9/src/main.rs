use std::fs;
use std::process;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
  x: i32,
  y: i32,
}

impl Position {
  fn boarders(&self, other: &Self) -> bool {
    println!("{}, {}", (self.x - other.x).abs(), (self.y - other.y).abs());  
    return (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1;
  }

  fn is_in_same_column(&self, other: &Self) -> bool {
    return self.y == other.y;
  }

  fn is_in_same_row(&self, other: &Self) -> bool {
    return self.x == other.x;
  }

  // Assume that we need to move to follow (we already checked this)
  fn follow(&mut self, head: &Self) {
    println!("Eval follow: {:?} -> {:?}", head, self);

    if self.is_in_same_column(&head) || self.is_in_same_row(&head) {
      self.y += (head.y-self.y)/2;
      self.x += (head.x - self.x)/2;
    } else {
      if (self.x - head.x).abs() == 2 {
        self.y += head.y-self.y;
        self.x += (head.x - self.x)/2;    
      } else {
        self.y += (head.y-self.y)/2;
        self.x += head.x - self.x;
      }
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

fn calculate_tail_visits(path_to_data : &str) -> u32 {
  //let moves = get_moves(path_to_data);

  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let moves:Vec<Move> = contents.lines().map(|line| Move::new(line)).collect();

  let mut visited:Vec<Position> = Vec::new();

  // Start in (0, 0), positive axis is right and up
  let mut head = Position { x: 0, y: 0 };
  let mut tail = Position { x: 0, y: 0 };

  visited.push(tail.clone());

  for m in moves.iter() {

    println!("Move {:?} from {:?}", m.direction, head);

    for step in 0..m.distance {
      match &m.direction {
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
        Direction::Right => head.x += 1,
        Direction::Left => head.x -= 1,
      }

      if !tail.boarders(&head) {
        tail.follow(&head);
      } else {
        //println!("Still close to the head!");
      }
      println!("Head: {:?}, Tail: {:?}", head, tail);
      println!("---------");

      if !visited.contains(&tail) {
        visited.push(tail.clone());
      }
    }
  }

  return visited.len() as u32;
}


fn main() {
  let pos = calculate_tail_visits("test.txt");
  assert!(pos == 13, "The number of positions the tail visited is not correct");

  let pos = calculate_tail_visits("data.txt");
  println!("The number of positions the tail visited is: {}", pos);
  assert!(pos == 6376, "The number of positions the tail visited is not correct");

  /*let size = calculate_max_scenic_score("test.txt");
  assert!(size == 8, "The test scenic score is not correct!");

  let size = calculate_max_scenic_score("data.txt");
  println!("Max scenic score is: {}", size);
  assert!(size == 211680, "The data scenic score is not correct!");*/
}
