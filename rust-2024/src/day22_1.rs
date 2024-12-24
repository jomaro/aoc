
use std::io::{self, BufRead};

const ROUNDS: u64 = 2_000;
const PRUNER: u64 = (1 << 24) -1;

pub fn solve() -> () {
    let numbers: Vec<u64> =
        io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

    let result: u64 = numbers.iter().map(|n| {
        let e = jump(*n, ROUNDS);

        println!("{} {}", n, e);

        e
    }
    ).sum();

    println!("{}", result);

    ()
}

fn jump(number: u64, n: u64) -> u64 {
    let mut result = number;

    for _ in 0..n {
        result = next(result);
    }

    result
}

fn next(n: u64) -> u64 {
    let mut s = n;

    s = ((s << 6) ^ s) & PRUNER;
    s = ((s >> 5) ^ s) & PRUNER;
    s = ((s << 11) ^ s) & PRUNER;

    s
}
