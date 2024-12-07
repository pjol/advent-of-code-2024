use std::path::PathBuf;

use crate::days;



fn get_parts(day: u8) -> (&'static dyn Fn() -> i32, &'static dyn Fn() -> i32) {
  match day {
    1 => return (&days::day1::part1, &days::day1::part2),
    2 => return (&days::day2::part1, &days::day2::part2),
    3 => return (&days::day3::part1, &days::day3::part2),
    _ => panic!("Day not available.")
  }
}

pub struct Day {
  pub number: u8,
  pub part1: &'static dyn Fn() -> i32,
  pub part2: &'static dyn Fn() -> i32,
}

impl Day {
  pub fn new(number: u8) -> Self {
    let parts = get_parts(number);
    Self {
      number,
      part1: parts.0,
      part2: parts.1
    }
  }

  pub fn display(&self) {
    println!("Day {:?}:", self.number);
    println!("  Part 1: {:?}", (self.part1)());
    println!("  Part 2: {:?}", (self.part2)());
  }
}

pub fn get_input_path(day: u8) -> String {
  let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  p.push(format!("inputs/input{:?}.csv", day));
  p.display().to_string()
}