use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, BufRead},
};

static GRID_FULL: &str = "#";
static GRID_EMPTY: &str = ".";

pub fn solve() -> () {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let robots: Vec<(i64, i64, i64, i64)> = io::stdin()
        .lock()
        .lines()
        .map(|line| extract(&re, &line.unwrap()))
        .collect();

    // println!("{:?}", robots);

    // let y: i64 = 11;
    // let x: i64 = 7;
    let y: i64 = 103;
    let x: i64 = 101;

    let min: i64 = 1;
    let max: i64 = 10000;
    // let min: i64 = 1000;
    // let max: i64 = 1000000;

    let mut line: Vec<&str> = Vec::with_capacity(x as usize);
    unsafe {
        line.set_len(x as usize);
    }

    for round in min..=max {
        let grid = calculate_grid(round, &robots, x, y);

        if candidate(&grid) {
            println!("\n\n[{}]\n", round);
            for j in 0..y {
                for i in 0..x {
                    let c = if grid.contains_key(&(i, j)) {
                        GRID_FULL
                    } else {
                        GRID_EMPTY
                    };

                    line[i as usize] = c;
                }

                println!("{}", line.join(""))
            }
        }
    }

    ()
}

fn candidate(grid: &HashMap<(i64, i64), i64>) -> bool {
    for ((i, j), _) in grid {
        let b = grid.contains_key(&(*i, j + 1))
            && grid.contains_key(&(*i, j+2))
            && grid.contains_key(&(i + 1, *j))
            && grid.contains_key(&(i + 1, j + 1))
            && grid.contains_key(&(i + 1, j + 2))
            && grid.contains_key(&(i + 2, *j))
            && grid.contains_key(&(i + 2, j + 1))
            && grid.contains_key(&(i + 2, j + 2));

        if b {
            return true;
        }
    }

    return false;
}

fn calculate_grid(
    round: i64,
    robots: &Vec<(i64, i64, i64, i64)>,
    x: i64,
    y: i64,
) -> HashMap<(i64, i64), i64> {
    let mut grid: HashMap<(i64, i64), i64> = HashMap::new();

    for (i, j, vi, vj) in robots {
        let i = (i + round * vi).rem_euclid(x);
        let j = (j + round * vj).rem_euclid(y);

        grid.entry((i, j))
            .and_modify(|entry| {
                *entry += 1;
            })
            .or_insert(1);
    }

    grid
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
