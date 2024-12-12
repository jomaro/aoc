use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn solve() -> () {
    let grid: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let y = grid.len() as i64;
    let x = grid[0].len() as i64;

    let mut visited: HashSet<(i64, i64)> = HashSet::new();

    let mut total = 0i64;

    for j in 0..y {
        for i in 0..x {
            let (a, p) = search(
                &grid,
                y,
                x,
                j,
                i,
                grid[j as usize][i as usize],
                &mut visited,
            );

            println!("{} {}", a, p);

            total += a * p;
        }
    }

    println!("{}", total);

    ()
}

fn search(
    grid: &Vec<Vec<char>>,
    y: i64,
    x: i64,
    j: i64,
    i: i64,
    base: char,
    visited: &mut HashSet<(i64, i64)>,
) -> (i64, i64) {
    if j < 0 || j >= y || i < 0 || i >= x {
        return (0, 1);
    }

    if grid[j as usize][i as usize] != base {
        return (0, 1);
    }

    if visited.contains(&(j, i)) {
        return (0, 0);
    }

    visited.insert((j, i));

    let (a1, p1) = search(grid, y, x, j - 1, i, base, visited);
    let (a2, p2) = search(grid, y, x, j + 1, i, base, visited);
    let (a3, p3) = search(grid, y, x, j, i - 1, base, visited);
    let (a4, p4) = search(grid, y, x, j, i + 1, base, visited);

    (1 + a1 + a2 + a3 + a4, p1 + p2 + p3 + p4)
}
