use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

pub fn solve() -> () {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let robots: Vec<(i64, i64, i64, i64)> = io::stdin()
        .lock()
        .lines()
        .map(|line| extract(&re, &line.unwrap()))
        .collect();

    println!("{:?}", robots);

    // let y: i64 = 11;
    // let x: i64 = 7;
    let y: i64 = 103;
    let x: i64 = 101;

    let rounds = 100;

    let mut grid: HashMap<(i64, i64), i64> = HashMap::new();

    for (i, j, vi, vj) in robots {
        let i = (i + rounds * vi).rem_euclid(x);
        let j = (j + rounds * vj).rem_euclid(y);

        grid.entry((i, j))
            .and_modify(|entry| {
                *entry += 1;
            })
            .or_insert(1);
    }

    println!("{:?}", grid);

    let mut result: i64 = 1;

    let quadrants = vec![
        (0..(x / 2), (0..(y / 2))),
        (0..(x / 2), (((y / 2) + 1)..y)),
        (((x / 2) + 1)..x, (0..(y / 2))),
        (((x / 2) + 1)..x, (((y / 2) + 1)..y)),
    ];

    for (rx, ry) in quadrants {
        result *= grid
            .iter()
            .map(|((i, j), v)| {
                if rx.contains(i) && ry.contains(j) {
                    return *v;
                }

                0i64
            })
            .sum::<i64>();
    }

    println!("{}", result);

    ()
}

fn extract(r: &Regex, s: &String) -> (i64, i64, i64, i64) {
    r.captures_iter(s)
        .next()
        .map(|c| c.extract())
        .map(|(_, [a, b, c, d])| {
            let a = a.parse::<i64>().unwrap();
            let b = b.parse::<i64>().unwrap();
            let c = c.parse::<i64>().unwrap();
            let d = d.parse::<i64>().unwrap();

            (a, b, c, d)
        })
        .unwrap()
}
