
use std::io::{self, BufRead};


pub fn solve() -> () {
    let value: u32 = io::stdin().lock().lines()
        // .take(3)
        .map(|line| line.unwrap())
        .map(|line| {
            get_first_number(line.as_ref()) * 10 + get_last_number(line.as_ref())
        })
        .sum();


    println!("{}", value);

    ()
}

fn get_first_number(string: &str) -> u32 {
    let number: char = string.chars().find(|&c| c >= '0' && c <= '9').unwrap();

    get_number(number)
}

fn get_last_number(string: &str) -> u32 {
    let number: char = string.chars().rfind(|&c| c >= '0' && c <= '9').unwrap();

    get_number(number)
}

fn get_number(capture: char) -> u32 {
    match capture {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => panic!("at the disco"),
    }
}
