use regex::Regex;
use std::io::{self, BufRead};

const OFFSET: i64 = 10000000000000;

pub fn solve() -> () {
    let machines: Vec<String> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .filter(|line| !line.is_empty())
        .collect();

    let ra = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let rb: Regex = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let rp: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let counter: i64 = machines
        .chunks_exact(3)
        .map(|vec| {
            let (ax, ay) = extract(&ra, &vec[0]);

            let (bx, by) = extract(&rb, &vec[1]);

            let (px, py) = extract(&rp, &vec[2]);

            (ax, ay, bx, by, OFFSET + px, OFFSET + py)
        })
        .map(|input| solve_for(input.0, input.1, input.2, input.3, input.4, input.5))
        .sum();

    println!("{}", counter);

    ()
}

fn solve_for(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> i64 {
    let i = (py as f64 - (((by * px) as f64) / bx as f64))
        / (ay as f64 - ((by * ax) as f64 / bx as f64));

    let j = (px as f64 - ax as f64 * i) / bx as f64;

    let i = i.round() as i64;
    let j = j.round() as i64;

    if px == ax * i + bx * j && py == ay * i + by * j {
        return 3 * i + j;
    }

    0
}

fn extract(r: &Regex, s: &String) -> (i64, i64) {
    r.captures_iter(s)
        .next()
        .map(|c| c.extract())
        .map(|(_, [a, b])| {
            let a = a.parse::<i64>().unwrap();
            let b = b.parse::<i64>().unwrap();

            (a, b)
        })
        .unwrap()
}

// px = ax * i + bx * j

// py = ay * i + by * j
// j = (px - ax * i) / bx

// py = ay * i + by * ((px - ax * i) / bx)
// py = ay * i + by * (px/bx - ax/bx * i)
// py = ay * i + (by * px/bx) - (by*ax/bx) * i
// py - (by * px/bx) = ay * i - (by*ax/bx) * i
// py - (by * px/bx) = (ay - (by*ax/bx)) * i
// (py - (by * px/bx) ) / (ay - (by*ax/bx))  = i

// px = ax * i + bx * j
// py = ay * i + by * j
// j = (px - ax * i) / bx
