use std::collections::HashMap;

use crate::utils::get_input_path;

pub fn part1() -> i32 {
  let path = get_input_path(5);
  let mut reader = csv::ReaderBuilder::new()
      .has_headers(false)
      .flexible(true)
      .from_path(path)
      .unwrap();

  let mut before: HashMap<u8, Vec<u8>> = HashMap::new();
  let mut after: HashMap<u8, Vec<u8>> = HashMap::new();

  let mut rows = reader.records();
  let mut rules = true;
  let mut total: i32 = 0;
  while let Some(Ok(row)) = rows.next() {
    if row[0].chars().nth(2).unwrap_or('0') != '|' { rules = false }
    if rules {
      let s: Vec<u8> = row[0].split("|").map(|x| x.parse().unwrap()).collect();
      let b = s[0];
      let a = s[1];
      before.entry(b).or_insert(Vec::new()).push(a);
      after.entry(a).or_insert(Vec::new()).push(b);
    } else {
      let nums: Vec<u8> = row.iter().map(|x| x.parse().unwrap()).collect();
      let mut is_ok = true;
      for (index, num) in nums.iter().enumerate() {
        if !is_ok { break }
        let before_list = before.get(num).unwrap();
        let after_list = after.get(num).unwrap();
        for (i, n) in nums.iter().enumerate() {
          if i == index { continue }
          if i < index && before_list.contains(n) { is_ok = false; break }
          if i > index && after_list.contains(n) { is_ok = false; break }
        }
      }
      if is_ok {
        total += nums[(nums.len() - 1) / 2] as i32
      }
    }
  }

  return total
}


pub fn part2() -> i32 {
  let path = get_input_path(5);
  let mut reader = csv::ReaderBuilder::new()
      .has_headers(false)
      .flexible(true)
      .from_path(path)
      .unwrap();

  let mut before: HashMap<u8, Vec<u8>> = HashMap::new();
  let mut after: HashMap<u8, Vec<u8>> = HashMap::new();

  let mut rows = reader.records();
  let mut rules = true;
  let mut total: i32 = 0;
  while let Some(Ok(row)) = rows.next() {
    if row[0].chars().nth(2).unwrap_or('0') != '|' { rules = false }
    if rules {
      let s: Vec<u8> = row[0].split("|").map(|x| x.parse().unwrap()).collect();
      let b = s[0];
      let a = s[1];
      before.entry(b).or_insert(Vec::new()).push(a);
      after.entry(a).or_insert(Vec::new()).push(b);
    } else {
      let nums: Vec<u8> = row.iter().map(|x| x.parse().unwrap()).collect();
      let mut is_ok = true;
      for (index, num) in nums.iter().enumerate() {
        if !is_ok { break }
        let before_list = before.get(num).unwrap();
        let after_list = after.get(num).unwrap();
        for (i, n) in nums.iter().enumerate() {
          if i == index { continue }
          if i < index && before_list.contains(n) { is_ok = false; break }
          if i > index && after_list.contains(n) { is_ok = false; break }
        }
      }
      if !is_ok {
        let mut new_nums: Vec<u8> = Vec::new();
        for (index, num) in nums.iter().enumerate() {
          if index == 0 { new_nums.push(*num); continue }
          let before_list = after.get(num).unwrap();
          for (i, n) in new_nums.clone().iter().enumerate() {
            if before_list.contains(n) { new_nums.insert(i, *num); break }
            if i == new_nums.len() -  1 { new_nums.push(*num) }
          }
        }

        total += new_nums[(new_nums.len() - 1) / 2] as i32
      }
    }
  }

  return total
}