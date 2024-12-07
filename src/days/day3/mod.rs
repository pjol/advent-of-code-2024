use std::fs;

use regex::Regex;

use crate::utils::get_input_path;

pub fn part1() -> i32 {
  let path =  get_input_path(3);
  let file = fs::read_to_string(path).unwrap();

  let re: Regex = Regex::new(r"mul\(\d{1,3},\d{1,3}\)").unwrap();
  let num_re: Regex = Regex::new(r"\d{1,3}").unwrap();
  let mut total = 0;

  let matches = re.find_iter(file.as_str());
  matches.for_each(|m| {
    let mut num_matches = num_re.find_iter(m.as_str());
    let n1: i32 = num_matches.next().unwrap().as_str().parse().unwrap();
    let n2: i32 = num_matches.next().unwrap().as_str().parse().unwrap();
    total += n1 * n2;
  });

  return total
}


pub fn part2() -> i32 {
  0
}