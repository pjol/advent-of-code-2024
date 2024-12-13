use std::collections::HashMap;

use crate::utils::get_input_path;

struct Traverser {
  visited: HashMap<(i32, i32), Vec<(i32, i32)>>,
  map: Vec<Vec<char>>,
  current_pos: (i32, i32),
  direction: (i32, i32)
}

impl Traverser {
  fn new(map: Vec<Vec<char>>) -> Self {

    let mut current_pos: (i32, i32) = (0, 0);
    for (x, row) in map.iter().enumerate() {
      for (y, c) in row.iter().enumerate() {
        if *c == '^' {
          current_pos = (x as i32, y as i32)
        }
      }
    }


    return Traverser{
      visited: HashMap::new(),
      map,
      current_pos,
      direction: (-1, 0)
    }
  }


  fn traverse(&mut self) -> Result<HashMap<(i32, i32), Vec<(i32, i32)>>, String> {
    loop {
      let mut next_pos = (self.current_pos.0 + self.direction.0, self.current_pos.1 + self.direction.1);
      if next_pos.0 < 0
      || next_pos.0 > self.map.len() as i32 - 1
      || next_pos.1 < 0
      || next_pos.1 > self.map[0].len() as i32 - 1
      {
        self.visited.entry(self.current_pos).or_insert(Vec::new()).push(self.direction);
        break
      }

      while self.map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
        self.direction = turn(self.direction);
        next_pos = (self.current_pos.0 + self.direction.0, self.current_pos.1 + self.direction.1);
      }

      if self.visited.entry(self.current_pos).or_insert(Vec::new()).contains(&self.direction) {
        return Err(String::from("in loop"))
      }
      self.visited.entry(self.current_pos).or_insert(Vec::new()).push(self.direction);

      self.current_pos = next_pos;
    }

    return Ok(self.visited.clone())
  }
}


pub fn part1() -> i32 {
  let path = get_input_path(6);
  let mut reader = csv::ReaderBuilder::new()
      .has_headers(false)
      .from_path(path)
      .unwrap();



  let mut records = reader.records();
  let mut map: Vec<Vec<char>> = Vec::new();

  while let Some(Ok(row)) = records.next() {
    map.push(row[0].chars().collect());
  }

  let mut traverser = Traverser::new(map);
  let visited = traverser.traverse().unwrap();

  return visited.into_keys().len() as i32
}


pub fn part2() -> i32 {
  let ans = std::env::var("CACHED_D6P2");
  if ans.is_ok() {
    return ans.unwrap().parse().unwrap()
  }
  let path = get_input_path(6);
  let mut reader = csv::ReaderBuilder::new()
      .has_headers(false)
      .from_path(path)
      .unwrap();
  let mut records = reader.records();
  let mut map: Vec<Vec<char>> = Vec::new();

  while let Some(Ok(row)) = records.next() {
    map.push(row[0].chars().collect());
  }

  let mut traverser = Traverser::new(map.clone());
  let visited = traverser.traverse().unwrap();

  let visited_spots: Vec<(i32, i32)> = visited.into_keys().collect();
  let mut total = 0;
  for spot in visited_spots {
    let mut new_map = map.clone();
    new_map[spot.0 as usize][spot.1 as usize] = '#';
    let mut t = Traverser::new(new_map.clone());
    if t.traverse().is_err() {
      total += 1
    }
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