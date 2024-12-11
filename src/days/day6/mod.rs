use std::{collections::HashMap, hash::Hash, thread::current};

use crate::utils::get_input_path;


pub fn part1() -> i32 {
  let path = get_input_path(6);
  let mut reader = csv::ReaderBuilder::new()
      .has_headers(false)
      .from_path(path)
      .unwrap();
  let mut records = reader.records();

  let mut visited: HashMap<(i32, i32), bool> = HashMap::new();
  let mut map: Vec<Vec<char>> = Vec::new();
  let mut current_pos: (i32, i32) = (0, 0);
  let mut direction: (i32, i32) = (-1, 0);

  while let Some(Ok(row)) = records.next() {
    map.push(row[0].chars().collect());
    for (index, c) in row[0].chars().enumerate() {
      if c == '^' {
        current_pos = (map.len() as i32 - 1, index as i32)
      }
    }
  }

  loop {
    visited.insert(current_pos, true);
    let mut next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
    if next_pos.0 < 0
      || next_pos.0 > map.len() as i32 - 1
      || next_pos.1 < 0
      || next_pos.1 > map[0].len() as i32 - 1 {
        break
      }
    if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
      direction = turn(direction);
      next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
    }
    current_pos = next_pos;
  }
  return visited.into_keys().len() as i32
}


pub fn part2() -> i32 {
  let path = get_input_path(6);
  let mut reader = csv::ReaderBuilder::new()
      .has_headers(false)
      .from_path(path)
      .unwrap();
  let mut records = reader.records();

  let mut visited: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
  let mut map: Vec<Vec<char>> = Vec::new();
  let mut current_pos: (i32, i32) = (0, 0);
  let mut direction: (i32, i32) = (-1, 0);
  let mut total = 0;

  while let Some(Ok(row)) = records.next() {
    map.push(row[0].chars().collect());
    for (index, c) in row[0].chars().enumerate() {
      if c == '^' {
        current_pos = (map.len() as i32 - 1, index as i32)
      }
    }
  }

  loop {
    visited.entry(current_pos).or_insert(Vec::new()).push(direction);
    let mut next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
    if next_pos.0 < 0
      || next_pos.0 > map.len() as i32 - 1
      || next_pos.1 < 0
      || next_pos.1 > map[0].len() as i32 - 1
    {
        break
    }

    if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
      direction = turn(direction);
      next_pos = (current_pos.0 + direction.0, current_pos.1 + direction.1);
    } else {
      let mut nested_visited: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();
      let mut nested_map = map.clone();
      nested_map[next_pos.0 as usize][next_pos.1 as usize] = '#';
      let mut nested_direction = direction.clone();
      let mut nested_pos = current_pos.clone();
      loop {
        if nested_visited.entry(nested_pos).or_insert(Vec::new()).contains(&nested_direction) {
          total += 1;
          break
        }
        nested_visited.entry(nested_pos).or_insert(Vec::new()).push(nested_direction);
        let mut nested_next = (nested_pos.0 + nested_direction.0, nested_pos.1 + nested_direction.1);
        if nested_next.0 < 0
          || nested_next.0 > nested_map.len() as i32 - 1
          || nested_next.1 < 0
          || nested_next.1 > nested_map[0].len() as i32 - 1
        {
            break
        }

        if nested_map[nested_next.0 as usize][nested_next.1 as usize] == '#' {
          nested_direction = turn(nested_direction);
          nested_next = (nested_pos.0 + nested_direction.0, nested_pos.1 + nested_direction.1);
        }

        nested_pos = nested_next;
      }

    }
    current_pos = next_pos;
  }
  return total
}

fn turn(current: (i32, i32)) -> (i32, i32) {
  match current {
    (-1, 0) => (0, 1),
    (0, 1) => (1, 0),
    (1, 0) => (0, -1),
    (0, -1) => (-1, 0),
    _ => (0, 0)
  }
}