

use std::io::{self, BufRead};

pub fn solve() {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    let mut current: u64 = 0;
    let mut elfs: Vec<u64> = Vec::new();

    while handle.read_line(&mut buffer).unwrap() != 0 {
        let line = buffer.trim();

        if line.is_empty() {
            elfs.push(current);

            current = 0;
        } else {
            current += line.trim_end().parse::<u64>().unwrap();
        }

        buffer.clear();
    }

    if current != 0 {
        elfs.push(current);
        // current = 0;
    }

    elfs.sort();

    elfs.reverse();

    println!("{}", elfs.into_iter().take(3).sum::<u64>());
}
