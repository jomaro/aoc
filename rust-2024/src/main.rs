use std::env;

mod utils;

mod day10_1;
mod day10_2;
mod day11_1;
mod day11_2;
mod day12_1;
mod day12_2;
mod day13_1;
mod day13_2;
mod day14_1;
mod day14_2;
mod day15_1;
mod day15_2;
mod day16_1;
mod day16_2;
mod day17_1;
mod day17_2;
mod day18_1;
mod day18_2;
mod day19_1;
mod day19_2;
mod day1_1;
mod day1_2;
mod day20_1;
mod day20_2;
mod day22_1;
mod day22_2;
mod day2_1;
mod day2_2;
mod day3_1;
mod day3_2;
mod day4_1;
mod day4_2;
mod day5_1;
mod day5_2;
mod day6_1;
mod day6_2;
mod day7_1;
mod day7_2;
mod day8_1;
mod day8_2;
mod day9_1;
mod day9_2;

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
        "aoc6-1" => day6_1::solve(),
        "aoc6-2" => day6_2::solve(),
        "aoc7-1" => day7_1::solve(),
        "aoc7-2" => day7_2::solve(),
        "aoc8-1" => day8_1::solve(),
        "aoc8-2" => day8_2::solve(),
        "aoc9-1" => day9_1::solve(),
        "aoc9-2" => day9_2::solve(),
        "aoc10-1" => day10_1::solve(),
        "aoc10-2" => day10_2::solve(),
        "aoc11-1" => day11_1::solve(),
        "aoc11-2" => day11_2::solve(),
        "aoc12-1" => day12_1::solve(),
        "aoc12-2" => day12_2::solve(),
        "aoc13-1" => day13_1::solve(),
        "aoc13-2" => day13_2::solve(),
        "aoc14-1" => day14_1::solve(),
        "aoc14-2" => day14_2::solve(),
        "aoc15-1" => day15_1::solve(),
        "aoc15-2" => day15_2::solve(),
        "aoc16-1" => day16_1::solve(),
        "aoc16-2" => day16_2::solve(),
        "aoc17-1" => day17_1::solve(),
        "aoc17-2" => day17_2::solve(),
        "aoc18-1" => day18_1::solve(),
        "aoc18-2" => day18_2::solve(),
        "aoc19-1" => day19_1::solve(),
        "aoc19-2" => day19_2::solve(),
        "aoc20-1" => day20_1::solve(),
        "aoc20-2" => day20_2::solve(),
        "aoc22-1" => day22_1::solve(),
        "aoc22-2" => day22_2::solve(),

        _ => panic!("Parameter invalid!!"),
    }
}
