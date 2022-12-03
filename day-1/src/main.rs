use std::fs;
use std::process;

#[derive(Debug)]
struct Elf {
  calories: u32
}

impl Elf {
  pub fn new(cal_string: String) -> Self {
      
    let mut total_cals = 0;
    let cals = cal_string.split("\n");

    for cal in cals {
      let item_cal : u32 = cal.parse().unwrap();
      total_cals += item_cal;
    }

      Elf {
          calories: total_cals
      }
  }
}

fn get_elves(path_to_data : &str) -> Vec<Elf> {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let split = contents.split("\n\n");

  let mut elves = Vec::new();

  for s in split {
    let elf = Elf::new(s.to_string());
    elves.push(elf);
  }

  println!("Parsed {} elves!", elves.len());

  return elves;
}

fn get_max_calories(path_to_data: &str) -> u32 {
  let elves = get_elves(path_to_data);

   let max = elves.iter().max_by_key(|e| e.calories).unwrap();

  return max.calories;
}

fn get_max_3_calories(path_to_data: &str) -> u32 {
  let mut elves = get_elves(path_to_data);

  elves.sort_by(|a, b| b.calories.cmp(&a.calories));
 
  let mut top3sum = 0;
  for n in 0..3 {
    top3sum += elves[n].calories;
  }

  return top3sum;
}

fn main() {
    let max_cals = get_max_calories("test.txt");
    assert!(max_cals == 24000, "Test max cals is wrong!");

    let max_cals = get_max_calories("data.txt");
    println!("The Elf with the max calories has {} cals", max_cals);
    assert!(max_cals == 68802, "Data max cals is wrong!");

    let max_cals = get_max_3_calories("test.txt");
    assert!(max_cals == 45000, "Test 3 max cals is wrong!");

    let max_cals = get_max_3_calories("data.txt");
    println!("The 3 top elves has {} cals in total", max_cals);
    assert!(max_cals == 205370, "Data 3 max cals is wrong!");

}
