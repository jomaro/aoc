use std::collections::{HashMap, HashSet};
use std::io::{self, BufRead};
use std::vec;

pub fn solve() -> () {
    let grid: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let y = grid.len();
    let x = grid[0].len();

    let mut antenas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    for j in 0..y {
        for i in 0..x {
            if grid[j][i] != '.' {
                antenas
                    .entry(grid[j][i])
                    .and_modify(|e| e.push((j, i)))
                    .or_insert(vec![(j, i)]);
            }
        }
    }

    println!("{:?}", antenas);

    let mut set: HashSet<(i64, i64)> = HashSet::new();

    for (_type, coord) in antenas {
        let size = coord.len();

        for (j, i) in &coord {
            set.insert((*j as i64, *i as i64));
        }

        for i in 0..(size - 1) {
            for j in (i + 1)..size {
                let p1 = coord[i];
                let p2 = coord[j];

                let dy = p2.0 as i64 - p1.0 as i64;
                let dx = p2.1 as i64 - p1.1 as i64;

                let mut e1 = (p2.0 as i64 + dy, p2.1 as i64 + dx);
                let mut e2 = (p1.0 as i64 - dy, p1.1 as i64 - dx);
                println!("{:?} {:?} {:?} {:?}", p1, p2, e1, e2);

                while validate(e1, y as i64, x as i64) {
                    set.insert(e1);

                    e1 = (e1.0 as i64 + dy, e1.1 as i64 + dx);
                }

                while validate(e2, y as i64, x as i64) {
                    set.insert(e2);

                    e2 = (e2.0 as i64 - dy, e2.1 as i64 - dx);
                }
            }
        }
    }

    println!("{}", set.len());

    ()
}

fn validate((j, i): (i64, i64), y: i64, x: i64) -> bool {
    !(j < 0 || j >= y || i < 0 || i >= x)
}
