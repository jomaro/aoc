use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn solve() -> () {
    let mut grid: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    let y = grid.len() as i64;
    let x = grid[0].len() as i64;

    let guard: Vec<char> = "v<>^".chars().collect();

    let mut position: Option<(char, i64, i64)> = None;

    for j in 0..y {
        for i in 0..x {
            if guard.contains(&grid[j as usize][i as usize]) {
                position = Some((grid[j as usize][i as usize], j, i));

                grid[j as usize][i as usize] = '.';
            }
        }
    }

    let mut position: (char, i64, i64) = position.unwrap();

    let original_position = position.clone();

    let mut positions: HashSet<(i64, i64)> = HashSet::new();

    loop {
        let (_, nj, ni) = next(&position);

        if nj < 0 || nj >= y || ni < 0 || ni >= x {
            break;
        }

        match grid[nj as usize][ni as usize] {
            '#' => {
                position = rotate(&position);
            }
            '.' => {
                position = next(&position);

                positions.insert((position.1, position.2));
            }
            tile => panic!("unknown tile type {}", tile),
        }
    }

    let counter: i64 = positions
        .iter()
        .map(|(j, i)| {
            let mut g = grid.clone();

            g[*j as usize][*i as usize] = '#';

            run(&original_position, y, x, &g)
        })
        .sum();

    println!("{}", counter);

    ()
}

fn run(position: &(char, i64, i64), y: i64, x: i64, grid: &Vec<Vec<char>>) -> i64 {
    let mut position = position.to_owned();

    let mut visited: HashSet<(char, i64, i64)> = HashSet::new();

    visited.insert(position);

    loop {
        let (_, nj, ni) = next(&position);

        if nj < 0 || nj >= y || ni < 0 || ni >= x {
            return 0;
        }

        match grid[nj as usize][ni as usize] {
            '#' => {
                position = rotate(&position);
                visited.insert(position);
            }
            '.' => {
                position = next(&position);

                if visited.contains(&position) {
                    return 1;
                }

                visited.insert(position);
            }
            tile => panic!("unknown tile type {}", tile),
        }
    }
}

fn rotate((orientation, j, i): &(char, i64, i64)) -> (char, i64, i64) {
    (orientation_next(*orientation), *j, *i)
}

fn orientation_next(orientation: char) -> char {
    match orientation {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => panic!("not found orientation {}", orientation),
    }
}

fn next((orientation, j, i): &(char, i64, i64)) -> (char, i64, i64) {
    match orientation {
        '^' => ('^', (*j) - 1, *i),
        '>' => ('>', *j, (*i) + 1),
        'v' => ('v', (*j) + 1, *i),
        '<' => ('<', *j, (*i) - 1),
        _ => panic!("not found orientation {}", orientation),
    }
}
