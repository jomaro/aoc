use std::io::{self, BufRead, Error};
use std::cmp::max;

#[derive(Eq, PartialEq, Hash, Debug)]
enum Color {
    Red,
    Blue,
    Green,
}


pub fn solve() -> () {
    let value: u32 = io::stdin().lock().lines()
        .map(get_game_id_if_valid_or_zero)
        .sum();

    println!("{}", value);
}

fn get_game_id_if_valid_or_zero(line_: Result<String, Error>) -> u32 {
    let line = line_.unwrap();

    let (red, blue, green): (u32, u32, u32) = line.split(&[':', ',', ';'][..])
        .map(|string: &str| string.trim())
        .skip(1)
        .map(parse_hand)
        .fold((0u32, 0u32, 0u32), |(red, blue, green): (u32, u32, u32), (color, value): (Color, u32)| {
            match color {
                Color::Red if red == 0 => (value, blue, green),
                Color::Red => (max(value, red), blue, green),
                Color::Blue if blue == 0 => (red, value, green),
                Color::Blue => (red, max(value, blue), green),
                Color::Green if green == 0 => (red, blue, value),
                Color::Green => (red, blue, max(value, green)),
            }
        });

    red * blue * green
}

fn parse_hand(hand: &str) -> (Color, u32) {
    let args = hand.split(' ').collect::<Vec<&str>>();

    return (parse_color(&args[1]), args[0].parse::<u32>().unwrap());
}

fn parse_color(color: &str) -> Color {
    match color {
        "red" => Color::Red,
        "blue" => Color::Blue,
        "green" => Color::Green,
        invalid => panic!("Unknown color: {}", invalid),
    }
}
