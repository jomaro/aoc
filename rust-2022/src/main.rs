#![feature(iter_array_chunks)]

#[macro_use]
extern crate lazy_static;

use std::env;

mod day1_1;
mod day1_2;
mod day2;
mod day3;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "aoc1-1" => day1_1::solve(),
        "aoc1-2" => day1_2::solve(),
        "aoc2-1" => day2::solve_part1(),
        "aoc2-2" => day2::solve_part2(),
        "aoc3-1" => day3::solve_part1(),
        "aoc3-2" => day3::solve_part2(),
        _ => panic!("Parameter invalid!!"),
    }
}
