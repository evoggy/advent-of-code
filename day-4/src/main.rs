use std::fs;
use std::process;

#[derive(Debug)]
struct Section {
  start: u32,
  end: u32
}

impl Section {
  pub fn new(section_string: &str) -> Self {  
    let mut sections = section_string.split("-");
    Section {
      start: sections.next().unwrap().parse().unwrap(),
      end: sections.next().unwrap().parse().unwrap()
    }
  }

  pub fn overlaps(&self, other: &Self) -> bool {  
    return 
      self.start >= other.start && self.end <= other.end ||
      self.start <= other.start && self.end >= other.end;
  }

  pub fn partially_overlaps(&self, other: &Self) -> bool {  
    return 
      self.start <= other.end && self.end >= other.start;
  }
}

fn nbr_of_overlapping_assignments(path_to_data : &str) -> i32 {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let number_of_overlapping_sections = 
    contents.split("\n")
      .map(|pair| {
        let mut sections = pair.split(",")
          .map(|section| {
            return Section::new(section);
          });
        if sections.next().unwrap().overlaps(&sections.next().unwrap()) {return 1;}
        else {return 0;}
      })
      .sum();

    //println!("{}", number_of_overlapping_sections);

    return number_of_overlapping_sections;
}

fn nbr_of_partially_overlapping_assignments(path_to_data : &str) -> i32 {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let number_of_overlapping_sections = 
    contents.split("\n")
      .map(|pair| {
        let mut sections = pair.split(",")
          .map(|section| {
            return Section::new(section);
          });
        if sections.next().unwrap().partially_overlaps(&sections.next().unwrap()) {return 1;}
        else {return 0;}
      })
      .sum();

    println!("{}", number_of_overlapping_sections);

    return number_of_overlapping_sections;
}

fn main() {
  let score = nbr_of_overlapping_assignments("test.txt");
  assert!(score == 2, "The test sum of overlapping teams is not correct!");

  let score = nbr_of_overlapping_assignments("data.txt");
  println!("The sum of individual items are: {}", score);
  assert!(score == 433, "The data sum of overlapping teams is not correct!");

  let score = nbr_of_partially_overlapping_assignments("test.txt");
  assert!(score == 4, "The test sum for badges is not correct!");

  let score = nbr_of_partially_overlapping_assignments("data.txt");
  println!("Your sum of badges are: {}", score);
  //assert!(score == 2609, "The data sum for badges is not correct!");
}
