use regex::Regex;
use std::io::{self, BufRead};

pub fn solve() -> () {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let value: i64 = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            re.captures_iter(&line.unwrap())
                .map(|c| c.extract())
                .map(|(_, [a, b])| {
                    let a = a.parse::<i64>().unwrap();
                    let b = b.parse::<i64>().unwrap();

                    println!("{:?} {:?} {:?}", a, b, a * b);

                    a * b
                })
                .sum::<i64>()
        })
        .sum();

    println!("{}", value);

    ()
}
