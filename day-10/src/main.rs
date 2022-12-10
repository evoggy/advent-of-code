use std::fs;
use std::process;

fn calculate_regx_value_over_time(path_to_data : &str) -> Vec<i32> {
  let contents = fs::read_to_string(path_to_data).unwrap_or_else(| err | {
    println!("Cannot file file!: {:?}", err);
    process::exit(1);
  });

  let mut signal_strength = 1;
  let mut signal_strength_over_time:Vec<i32> = Vec::new();

  for line in contents.lines() {
    let mut parts = line.split(" ");
    match (parts.next(), parts.next()) {
      (Some("noop"), None) => signal_strength_over_time.push(signal_strength),
      (Some("addx"), Some(nbr)) => {
        let new_signal_strength = signal_strength + nbr.parse::<i32>().unwrap();
        signal_strength_over_time.push(signal_strength);
        signal_strength_over_time.push(signal_strength);
        signal_strength = new_signal_strength;
      },
      (_,_) => unreachable!()
    }
  }

  return signal_strength_over_time;
}

fn calculate_signal_strength(path_to_data : &str) -> i32 {
  let total_signal_strength:i32 = calculate_regx_value_over_time(path_to_data).iter().enumerate()
    .map(|(i, &x)| {
      let cycle = i as i32 + 1;
      if cycle > 0 && (cycle == 20 || (cycle+20) % 40 == 0) {
        return cycle * x;
      }
      return 0;
    })
    .sum();

  return total_signal_strength;
}

fn render_sprite(path_to_data : &str) -> String {
  let mut render = String::new();
  for (i, signal) in calculate_regx_value_over_time(path_to_data).iter().enumerate() {
    let pixel = i as i32;

    if pixel % 40 >= signal - 1 && pixel % 40 <= signal + 1 {
      render.push_str("#");
    } else {
      render.push_str(".");
    }

    if pixel > 0 && (pixel + 1) % 40 == 0 {
      render.push_str("\n");
    }
  }
  return render;
}

fn main() {
  let size = calculate_signal_strength("test.txt");
  assert!(size == 13140, "The test signal strength is not correct!");

  let size = calculate_signal_strength("data.txt");
  println!("The signal strength is: {}", size);
  assert!(size == 14060, "The data signal strength is not correct!");

  let render = render_sprite("test.txt");
  let test_render = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....\n";
  assert!(render == test_render, "The test render was not correct");

  let render = render_sprite("data.txt");
  let data_render = "###...##..###..#..#.####.#..#.####...##.\n#..#.#..#.#..#.#.#..#....#.#..#.......#.\n#..#.#..#.#..#.##...###..##...###.....#.\n###..####.###..#.#..#....#.#..#.......#.\n#....#..#.#....#.#..#....#.#..#....#..#.\n#....#..#.#....#..#.#....#..#.####..##..\n";
  assert!(render == data_render, "The data render was not correct");
  println!("{}", render);
}
