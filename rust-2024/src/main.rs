
use std::env;

mod day1_1;
mod day1_2;
mod day2_1;
mod day2_2;
mod day3_1;
mod day3_2;
mod day4_1;
mod day4_2;
mod day5_1;
mod day5_2;


fn main() {
  let args: Vec<String> = env::args().collect();

  match args[1].as_str() {
    "aoc1-1" => day1_1::solve(),
    "aoc1-2" => day1_2::solve(),
    "aoc2-1" => day2_1::solve(),
    "aoc2-2" => day2_2::solve(),
    "aoc3-1" => day3_1::solve(),
    "aoc3-2" => day3_2::solve(),
    "aoc4-1" => day4_1::solve(),
    "aoc4-2" => day4_2::solve(),
    "aoc5-1" => day5_1::solve(),
    "aoc5-2" => day5_2::solve(),

    _ => panic!("Parameter invalid!!"),
  }
}
