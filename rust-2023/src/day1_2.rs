
use std::io::{self, BufRead};
use regex::Regex;

use phf::phf_map;

const NUMBERS: phf::Map<&'static str, u32> = phf_map! {
    "zero" => 0,
    "one" => 1,
    "two" => 2,
    "three" => 3,
    "four" => 4,
    "five" => 5,
    "six" => 6,
    "seven" => 7,
    "eight" => 8,
    "nine" => 9,
};

pub fn solve() -> () {
    let re: Regex = Regex::new(format!("({}|0|1|2|3|4|5|6|7|8|9)", NUMBERS.keys().map(|s| s.to_string()).collect::<Vec<String>>().join("|")).as_ref()).unwrap();
    let re_rev: Regex = Regex::new(format!("({}|0|1|2|3|4|5|6|7|8|9)", NUMBERS.keys().map(|s| s.to_string().chars().rev().collect::<String>()).collect::<Vec<String>>().join("|")).as_ref()).unwrap();

    let value: u32 = io::stdin().lock().lines()
        // .take(3)
        .map(|line| line.unwrap())
        .map(|line| {
            let e = get_first_number(line.as_ref(), &re) * 10 + get_last_number(line.as_ref(), &re_rev);

            // println!("p: {}", e);

            e
        })
        .sum();


    println!("{}", value);

    ()
}

fn get_first_number(string: &str, re: &Regex) -> u32 {
    let capture = re.find(string).unwrap().as_str();

    match capture {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "zero" => 0,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => panic!("at the disco"),
    }
}

fn get_last_number(string: &str, re: &Regex) -> u32 {
    let rev = string.chars().rev().collect::<String>();
    
    let capture = re.find(&rev).unwrap().as_str();

    match capture {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "orez" => 0,
        "eno" => 1,
        "owt" => 2,
        "eerht" => 3,
        "ruof" => 4,
        "evif" => 5,
        "xis" => 6,
        "neves" => 7,
        "thgie" => 8, // eight
        "enin" => 9,
        _ => panic!("at the disco"),
    }
}
