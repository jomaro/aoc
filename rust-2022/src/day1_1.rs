
use std::io::{self, BufRead};

pub fn solve() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut current: u64 = 0;
    let mut max: u64 = 0;

    while handle.read_line(&mut buffer).unwrap() != 0 {
        let line = buffer.trim();

        if line.is_empty() {
            if current > max {
                max = current;
            }

            current = 0;
        } else {
            current += line.trim_end().parse::<u64>().unwrap();
        }

        buffer.clear();
    }

    println!("{}", max);
}
