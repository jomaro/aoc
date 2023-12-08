use std::io::{self, BufRead, Error};

const RED: u32 = 12;
const BLUE: u32 = 14;
const GREEN: u32 = 13;


pub fn solve() -> () {
    let value: u32 = io::stdin().lock().lines().enumerate().map(get_game_id_if_valid_or_zero).sum();

    println!("{}", value);
}

fn get_game_id_if_valid_or_zero((index, line_): (usize, Result<String, Error>)) -> u32 {
    let line = line_.unwrap();

    let v: Vec<&str> = line.split(&[':', ',', ';'][..]).map(|string: &str| string.trim()).collect();

    if v[1..].iter().all(is_valid_hand) {
        (1 + index) as u32
    } else {
        0
    }
}

fn is_valid_hand(hand: &&str) -> bool {
    let args = hand.split(' ').collect::<Vec<&str>>();

    return args[0].parse::<u32>().unwrap() <= get_color_limit(&args[1]);
}

fn get_color_limit(color: &str) -> u32 {
    match color {
        "red" => RED,
        "blue" => BLUE,
        "green" => GREEN,
        invalid => panic!("Unknown color: {}", invalid),
    }
}
