use utils::Day;

mod days;
mod utils;

const DAYS_COMPLETED: u8 = 3;

fn main() {
  for n in 1..DAYS_COMPLETED + 1 {
    let day = Day::new(n);
    day.display();
  }
}