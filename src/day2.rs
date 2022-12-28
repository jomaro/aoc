

use std::io::{self, BufRead};
use std::collections::HashMap;
use std::cmp::{Eq, PartialEq};

#[derive(Eq, PartialEq, Hash, Clone)]
enum RPS {
    ROCK = 1,
    PAPER = 2,
    SCISORS = 3,
}

#[derive(Eq, PartialEq, Hash, Clone)]
enum OUTCOME {
    LOSE = 0,
    DRAW = 3,
    WIN = 6,
}

lazy_static!{
    static ref SCORES: HashMap<(RPS, RPS), u64> = vec![
        ((RPS::ROCK,    RPS::ROCK    ), 1u64),
        ((RPS::ROCK,    RPS::PAPER   ), 2   ),
        ((RPS::ROCK,    RPS::SCISORS ), 0   ),
        ((RPS::PAPER,   RPS::ROCK    ), 0   ),
        ((RPS::PAPER,   RPS::PAPER   ), 1   ),
        ((RPS::PAPER,   RPS::SCISORS ), 2   ),
        ((RPS::SCISORS, RPS::ROCK    ), 2   ),
        ((RPS::SCISORS, RPS::PAPER   ), 0   ),
        ((RPS::SCISORS, RPS::SCISORS ), 1   ),
    ].into_iter().collect();
}

lazy_static!{
    static ref STRATEGY_OUTCOME: HashMap<(RPS, OUTCOME), RPS> = vec![
        ((RPS::ROCK,    OUTCOME::LOSE), RPS::SCISORS),
        ((RPS::ROCK,    OUTCOME::DRAW), RPS::ROCK   ),
        ((RPS::ROCK,    OUTCOME::WIN ), RPS::PAPER  ),
        ((RPS::PAPER,   OUTCOME::LOSE), RPS::ROCK   ),
        ((RPS::PAPER,   OUTCOME::DRAW), RPS::PAPER  ),
        ((RPS::PAPER,   OUTCOME::WIN ), RPS::SCISORS),
        ((RPS::SCISORS, OUTCOME::LOSE), RPS::PAPER  ),
        ((RPS::SCISORS, OUTCOME::DRAW), RPS::SCISORS),
        ((RPS::SCISORS, OUTCOME::WIN ), RPS::ROCK   ),
    ].into_iter().collect();
}

pub fn solve_part1() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut points: u64 = 0;

    while handle.read_line(&mut buffer).unwrap() != 0 {
        if buffer.is_empty() {
            continue;
        }

        let values: _ = buffer.trim().split(" ").collect::<Vec<&str>>();
        let my_hand = normalize(&values[1]);
        let values = (normalize(&values[0]), my_hand.clone());

        points += my_hand as u64 + SCORES.get(&values).unwrap() * 3;

        buffer.clear();
    }

    println!("{}", points);
}

pub fn solve_part2() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut points: u64 = 0;

    while handle.read_line(&mut buffer).unwrap() != 0 {
        if buffer.is_empty() {
            continue;
        }

        let values: _ = buffer.trim().split(" ").collect::<Vec<&str>>();
        let strategy = normalize_second_column(&values[1]);
        let values = (normalize(&values[0]), strategy.clone());

        let my_hand = STRATEGY_OUTCOME.get(&values).unwrap().clone();

        points += my_hand as u64 + strategy as u64;

        buffer.clear();
    }

    println!("{}", points);
}

fn normalize(char: &str) -> RPS {
    match char {
        "A" => RPS::ROCK,
        "B" => RPS::PAPER,
        "C" => RPS::SCISORS,
        "X" => RPS::ROCK,
        "Y" => RPS::PAPER,
        "Z" => RPS::SCISORS,
        _ => panic!("Unknown input {}", char),
    }
}

fn normalize_second_column(char: &str) -> OUTCOME {
    match char {
        "X" => OUTCOME::LOSE,
        "Y" => OUTCOME::DRAW,
        "Z" => OUTCOME::WIN,
        _ => panic!("Unknown input {}", char),
    }
}