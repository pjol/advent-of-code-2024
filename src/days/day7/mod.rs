use crate::utils::get_input_path;

struct Calibrator {
  total: i64,
  equation: Vec<i64>
}

impl Calibrator {

  fn new(total: i64, equation: Vec<i64>) -> Self {
    return Calibrator {
      total,
      equation
    }
  }

  fn is_true(&self) -> bool {
    let mut eq = self.equation.clone();
    let num =  eq.pop();
    if num.is_none() { return self.total == 0 }
    let n = num.unwrap();

    let mut next: Vec<Calibrator> = Vec::new();
    if self.total % n == 0 && self.total != 0 {
      next.push(Calibrator { total: self.total / n, equation: eq.clone() })
    }
    next.push(Calibrator { total: self.total - n, equation: eq.clone() });

    for c in next {
      if c.is_true() { return true }
    }

    return false
  }
}

pub fn part1() -> i64 {
  let path = get_input_path(7);
  let mut reader = csv::ReaderBuilder::new()
      .has_headers(false)
      .from_path(path)
      .unwrap();



  let mut records = reader.records();
  let mut total = 0;
  while let Some(Ok(record)) = records.next() {
    let split: Vec<&str> = record[0].split(": ").collect();
    let calibrator_total: i64 = split[0].parse().unwrap();
    let calibrator_equation: Vec<i64> = split[1].split(" ").map(|x| x.parse().unwrap()).collect();
    if Calibrator::new(calibrator_total.clone(), calibrator_equation.clone()).is_true() { total += calibrator_total }
  }
  return total.try_into().unwrap()
}

pub fn part2() -> i32 {

  println!("0 mod 2: {:?}", 0 % 2);
  return 0
}