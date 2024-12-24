

use std::io::{self, BufRead};
use std::collections::HashMap;

const ROUNDS: u64 = 2_000;
const PRUNER: u64 = (1 << 24) -1;

pub fn solve() -> () {
    let buyers: Vec<u64> =
        io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap().parse().unwrap())
            .collect();

    let buyers_sequences: Vec<HashMap<(i8, i8, i8, i8), i8>> = buyers.iter().map(|n| {
        let prices = gen_prices(*n, ROUNDS);

        gen_sequences(&prices)
    }
    ).collect();

    let range = -9i8..=9;

    let mut found: i64 = 0;

    for a in range.clone() {
        for b in range.clone() {
            for c in range.clone() {
                for d in range.clone() {
                    let current: i64 = buyers_sequences.iter().map(|seq| *seq.get(&(a,b,c,d)).unwrap_or(&0i8) as i64).sum();

                    if current > found {
                        found = current;
                    }
                }
            }
        }
    }

    println!("{}", found);

    ()
}

fn gen_sequences(prices: &Vec<i8>) -> HashMap<(i8, i8, i8, i8), i8> {
    let mut sequences: HashMap<(i8, i8, i8, i8), i8> = HashMap::new();

    let mut i = prices.iter();

    let mut a = i.next().unwrap();
    let mut b = i.next().unwrap();
    let mut c = i.next().unwrap();
    let mut d = i.next().unwrap();

    for e in i {
        let key = (e-d, d-c, c-b, b-a);

        if !sequences.contains_key(&key) {
            sequences.insert(key, *e);
        }

        a = b;
        b = c;
        c = d;
        d = e;
    }

    sequences
}

fn gen_prices(number: u64, n: u64) -> Vec<i8> {
    let mut result = number;
    let mut prices: Vec<i8> = vec![(number % 10) as i8];

    for _ in 0..n {
        result = next(result);

        prices.push((result % 10) as i8);
    }

    prices
}

fn next(n: u64) -> u64 {
    let mut s = n;

    s = ((s << 6) ^ s) & PRUNER;
    s = ((s >> 5) ^ s) & PRUNER;
    s = ((s << 11) ^ s) & PRUNER;

    s
}
