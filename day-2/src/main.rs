use std::fs;
use std::process;

#[derive(PartialEq, Eq)]
enum Move {
  Rock,
  Paper,
  Scissors
}

impl Move {
  pub fn points(&self) -> u32 {
    match self {
      Self::Rock => 1,
      Self::Paper => 2,
      Self::Scissors => 3
    }
  }

  pub fn winning_move(&self) -> Move {
    match self {
      Self::Rock => Self::Paper,
      Self::Paper => Self::Scissors,
      Self::Scissors => Self::Rock
    }
  }

  pub fn drawing_move(&self) -> Move {
    match self {
      Self::Rock => Self::Rock,
      Self::Paper => Self::Paper,
      Self::Scissors => Self::Scissors
    }
  }

  pub fn loosing_move(&self) -> Move {
    match self {
      Self::Rock => Self::Scissors,
      Self::Paper => Self::Rock,
      Self::Scissors => Self::Paper
    }
  }
}


enum Strategy {
  ShouldWin,
  ShouldLoose,
  ShouldDraw
}

struct Round {
  challenge: Move,
  response: Move,
  strategy: Strategy
}

impl Round {

  fn decode_move(letter: &str) -> Move {
    match letter {
      "A" | "X" => Move::Rock,
      "B" | "Y" => Move::Paper,
      "C" | "Z" => Move::Scissors,
      _ => todo!()
    }
  }

  fn decode_strategy(letter: &str) -> Strategy {
    match letter {
      "X" => Strategy::ShouldLoose,
      "Y" => Strategy::ShouldDraw,
      "Z" => Strategy::ShouldWin,
      _ => todo!()
    }
  }

  pub fn score(&self) -> u32 {   
    let response_points = self.response.points();
    
    // Default lost
    let mut game_points = 0;

    if self.challenge.winning_move() == self.response {
      game_points = 6;
    }

    if self.challenge.drawing_move() == self.response {
      game_points = 3;
    }

    return response_points + game_points;
  }

  pub fn new(round: String) -> Self {
    let split = round.split(" ");
    let moves: Vec<&str> = split.collect();
      Round {
        challenge: Round::decode_move(moves[0]),
        response: Round::decode_move(moves[1]),
        strategy: Round::decode_strategy(moves[1]) // Same as response in step 1
      }
  }

  pub fn select_response_from_secret_strategy(&mut self) {
    self.response = match self.strategy {
      Strategy::ShouldDraw => self.challenge.drawing_move(),
      Strategy::ShouldLoose => self.challenge.loosing_move(),
      Strategy::ShouldWin => self.challenge.winning_move(),
    }
  }

}

fn run_game(path_to_data : &str) -> u32 {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let split = contents.split("\n");
  let mut rounds = Vec::new();

  for r in split {
    let round = Round::new(r.to_string());
    rounds.push(round);
  }

  println!("Got {} rounds", rounds.len());

  let mut total_points = 0;

  for round in rounds {
    total_points += round.score();
  }

  return total_points;
}

fn run_secret_strategy_game(path_to_data : &str) -> u32 {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let split = contents.split("\n");
  let mut rounds = Vec::new();

  for r in split {
    let round = Round::new(r.to_string());
    rounds.push(round);
  }

  println!("Got {} rounds", rounds.len());

  let mut total_points = 0;
  for mut round in rounds {
    round.select_response_from_secret_strategy();
    total_points += round.score();
  }

  return total_points;
}

fn main() {
  let score = run_game("test.txt");
  assert!(score == 15, "The test score is not correct!");

  let score = run_game("data.txt");
  println!("Your score is: {}", score);
  assert!(score == 15523, "The data score is not correct!");

  let score = run_secret_strategy_game("test.txt");
  assert!(score == 12, "The test score for secret game is not correct!");

  let score = run_secret_strategy_game("data.txt");
  println!("Your score is: {}", score);
  assert!(score == 15702, "The data score for secret game is not correct!");
}
