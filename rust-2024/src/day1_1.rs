use std::io::{self, BufRead};

pub fn solve() -> () {
    let (mut left, mut right): (Vec<_>, Vec<_>) = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let numbers: Vec<i64> = line
                .unwrap()
                .split("   ")
                .map(|number| number.parse::<i64>().unwrap())
                .collect();

            (numbers[0], numbers[1])
        })
        .unzip();

    left.sort_unstable();
    right.sort_unstable();

    let value: u64 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| (l - r).abs() as u64)
        .sum();

    println!("{}", value);

    ()
}

// 30047353
// 30047353
