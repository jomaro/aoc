use std::io::{self, BufRead};

pub fn solve() -> () {
    let mut line: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|line| {
            line.split(" ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .unwrap();

    let iterations = 25;

    for _ in 0..iterations {
        line = line
            .iter()
            .flat_map(|item| {
                if *item == 0 {
                    return vec![1];
                }

                let len = item.to_string().len() >> 1;

                if item.to_string().len() & 1 == 0 {
                    return vec![
                        item.to_string()[0..len].parse::<i64>().unwrap(),
                        item.to_string()[len..].parse::<i64>().unwrap(),
                    ];
                }

                vec![2024 * item]
            })
            .collect::<Vec<i64>>();
    }

    println!("{}", line.len());

    ()
}
