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

    let mut total = 0i64;

    let mut visited_a: HashSet<(i64, i64)> = HashSet::new();
    let mut visited_p: HashSet<(i64, i64)> = HashSet::new();

    for j in 0..y {
        for i in 0..x {
            let a = search_area_rec(
                &grid,
                y,
                x,
                j,
                i,
                grid[j as usize][i as usize],
                &mut visited_a,
            );
            let p = search_perimeter(
                &grid,
                y,
                x,
                j,
                i,
                grid[j as usize][i as usize],
                &mut visited_p,
            );

            total += a * p;
        }
    }

    println!("{}", total);

    ()
}

fn search_area_rec(
    grid: &Vec<Vec<char>>,
    y: i64,
    x: i64,
    j: i64,
    i: i64,
    base: char,
    visited: &mut HashSet<(i64, i64)>,
) -> i64 {
    if j < 0 || j >= y || i < 0 || i >= x {
        return 0;
    }

    if grid[j as usize][i as usize] != base {
        return 0;
    }

    if visited.contains(&(j, i)) {
        return 0;
    }

    visited.insert((j, i));

    let a1 = search_area_rec(grid, y, x, j - 1, i, base, visited);
    let a2 = search_area_rec(grid, y, x, j + 1, i, base, visited);
    let a3 = search_area_rec(grid, y, x, j, i - 1, base, visited);
    let a4 = search_area_rec(grid, y, x, j, i + 1, base, visited);

    1 + a1 + a2 + a3 + a4
}

fn search_perimeter(
    grid: &Vec<Vec<char>>,
    y: i64,
    x: i64,
    j: i64,
    i: i64,
    base: char,
    visited: &mut HashSet<(i64, i64)>,
) -> i64 {
    let mut fences: HashSet<(i64, i64, char)> = HashSet::new();

    search_perimeter_rec(grid, y, x, j, i, base, visited, &mut fences);

    for (j, i, dir) in fences.clone() {
        match dir {
            '^' | 'v' => {
                let mut c = i + 1;
                while fences.contains(&(j, c, dir)) {
                    fences.remove(&(j, c, dir));

                    c += 1;
                }
            }
            '>' | '<' => {
                let mut c = j + 1;
                while fences.contains(&(c, i, dir)) {
                    fences.remove(&(c, i, dir));

                    c += 1;
                }
            }
            o => {
                panic!("Unknown orientation {o}");
            }
        }
    }

    fences.len() as i64
}

fn search_perimeter_rec(
    grid: &Vec<Vec<char>>,
    y: i64,
    x: i64,
    j: i64,
    i: i64,
    base: char,
    visited: &mut HashSet<(i64, i64)>,
    fences: &mut HashSet<(i64, i64, char)>,
) -> bool {
    if j < 0 || j >= y || i < 0 || i >= x {
        return true;
    }

    if grid[j as usize][i as usize] != base {
        return true;
    }

    if visited.contains(&(j, i)) {
        return false;
    }

    visited.insert((j, i));

    if search_perimeter_rec(grid, y, x, j - 1, i, base, visited, fences) {
        fences.insert((j, i, '^'));
    }

    if search_perimeter_rec(grid, y, x, j + 1, i, base, visited, fences) {
        fences.insert((j, i, 'v'));
    }

    if search_perimeter_rec(grid, y, x, j, i - 1, base, visited, fences) {
        fences.insert((j, i, '<'));
    }

    if search_perimeter_rec(grid, y, x, j, i + 1, base, visited, fences) {
        fences.insert((j, i, '>'));
    }

    false
}
