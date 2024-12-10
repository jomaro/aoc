use std::io::{self, BufRead};

pub fn solve() -> () {
    let grid: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let y = grid.len() as i64;
    let x = grid[0].len() as i64;

    let result = solve_cross(&grid, x, y);

    println!("{}", result);

    ()
}

fn solve_cross(grid: &Vec<Vec<char>>, w: i64, h: i64) -> i64 {
    let mut counter = 0i64;

    for j in 1..(h - 1) {
        for i in 1..(w - 1) {
            if grid[j as usize][i as usize] != 'A' {
                continue;
            }

            if !((grid[(j + 1) as usize][(i + 1) as usize] == 'M'
                && grid[(j - 1) as usize][(i - 1) as usize] == 'S')
                || (grid[(j + 1) as usize][(i + 1) as usize] == 'S'
                    && grid[(j - 1) as usize][(i - 1) as usize] == 'M'))
            {
                continue;
            }

            if !((grid[(j + 1) as usize][(i - 1) as usize] == 'M'
                && grid[(j - 1) as usize][(i + 1) as usize] == 'S')
                || (grid[(j + 1) as usize][(i - 1) as usize] == 'S'
                    && grid[(j - 1) as usize][(i + 1) as usize] == 'M'))
            {
                continue;
            }

            counter += 1;
        }
    }

    counter
}
