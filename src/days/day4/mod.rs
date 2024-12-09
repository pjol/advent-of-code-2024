use crate::utils::get_input_path;

pub fn part1() -> i32 {
  let path = get_input_path(4);
  let mut reader = csv::ReaderBuilder::new()
      .has_headers(false)
      .from_path(path)
      .unwrap();

  let mut records = reader.records();
  let mut matrix: Vec<Vec<char>> = Vec::new();

  while let Some(record) = records.next() {
    let row = record.unwrap();
    let chars: Vec<char> = row[0].chars().collect();
    matrix.push(chars);
  }

  let mut count = 0;

  for (x, row) in matrix.iter().enumerate() {
    for (y, _) in row.iter().enumerate() {
      let cur: char = matrix[x][y];
      if cur != 'X' { continue }
      for dirx in -1..2 {
        for diry in -1..2 {
          let mut c = cur;
          let mut newx: i32 = x as i32 + dirx;
          let mut newy: i32 = y as i32 + diry;
          if newx < 0 || newy < 0 || newx > matrix.len() as i32 - 1 || newy > row.len() as i32 - 1 {
            continue
          }


          let mut check: char = matrix[newx as usize][newy as usize];
          while check == get_next(c) {
            c = get_next(c);
            if c == 'S' {
              count += 1;
              break
            }
            newx = newx as i32 + dirx;
            newy = newy as i32 + diry;
            if newx < 0 || newy < 0 || newx > matrix.len() as i32 - 1 || newy > row.len() as i32 - 1 {
              break
            }

            check = matrix[newx as usize][newy as usize];
          }
        }
      }
    }
  }


  return count
}

pub fn part2() -> i32 {
  let path = get_input_path(4);
  let mut reader = csv::ReaderBuilder::new()
      .has_headers(false)
      .from_path(path)
      .unwrap();

  let mut records = reader.records();
  let mut matrix: Vec<Vec<char>> = Vec::new();

  while let Some(record) = records.next() {
    let row = record.unwrap();
    let chars: Vec<char> = row[0].chars().collect();
    matrix.push(chars);
  }

  let mut count = 0;

  for (x, row) in matrix.iter().enumerate() {
    for (y, _) in row.iter().enumerate() {
      let cur: char = matrix[x][y];
      if cur != 'A' { continue }

      let mut ok = true;
      for n in [(-1, -1), (1, -1)] {
        if ok == false { break }
        let c1x = x as i32 + n.0;
        let c1y = y as i32 + n.1;
        if c1x < 0 || c1y < 0 || c1x > matrix.len() as i32 - 1 || c1y > row.len() as i32 - 1 {
          ok = false;
          continue
        }
        let c1 = matrix[c1x as usize][c1y as usize];

        let c2x = x as i32 + (n.0 * -1);
        let c2y = y as i32 + (n.1 * -1);
        if c2x < 0 || c2y < 0 || c2x > matrix.len() as i32 - 1 || c2y > row.len() as i32 - 1 {
          ok = false;
          continue
        }
        let c2 = matrix[c2x as usize][c2y as usize];

        if !((c1 == 'M' && c2 == 'S') || (c1 == 'S' && c2 == 'M')) {
          ok = false;
        }
      }
      if ok == true {
        count += 1
      }
    }
  }


  return count
}

fn get_next(cur: char) -> char {
  match cur {
    'X' => 'M',
    'M' => 'A',
    'A' => 'S',
    'S' => '1',
    _ => return '0'
  }
}