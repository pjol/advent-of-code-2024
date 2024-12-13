use utils::Day;
use dotenv::dotenv;

mod days;
mod utils;

const DAYS_COMPLETED: u8 = 7;

fn main() {
  let _ = dotenv();
  for n in 1..DAYS_COMPLETED + 1 {
    let day = Day::new(n);
    day.display();
  }
}