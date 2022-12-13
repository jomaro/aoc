
use std::env;

mod day1_1;
mod day1_2;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "aoc1-1" => day1_1::solve(),
        "aoc1-2" => day1_2::solve(),
        _ => panic!("Parameter invalid!!"),
    }
}
