use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn solve() -> () {
    let (left, right): (Vec<_>, Vec<_>) = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let numbers: Vec<u64> = line
                .unwrap()
                .split("   ")
                .map(|number| number.parse::<u64>().unwrap())
                .collect();

            (numbers[0], numbers[1])
        })
        .unzip();

    let right_frequencies = frequency(&right);

    let value: u64 = left
        .into_iter()
        .map(|l| l * *right_frequencies.get(&l).unwrap_or(&0u64))
        .sum();

    println!("{}", value);

    ()
}

fn frequency(v: &Vec<u64>) -> HashMap<u64, u64> {
    let mut m: HashMap<u64, u64> = HashMap::new();
    for x in v {
        *m.entry(*x).or_default() += 1;
    }

    m
}
