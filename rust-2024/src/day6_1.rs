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

    let mut counter = 1;

    let mut position: Option<(char, i64, i64)> = None;

    for j in 0..y {
        for i in 0..x {
            if guard.contains(&grid[j as usize][i as usize]) {
                println!("{} {} {}", grid[j as usize][i as usize], j, i);

                position = Some((grid[j as usize][i as usize], j, i));

                grid[j as usize][i as usize] = 'X';
            }
        }
    }

    let mut position: (char, i64, i64) = position.unwrap();

    loop {
        let (nj, ni) = next(&position);

        if nj < 0 || nj >= y || ni < 0 || ni >= x {
            break;
        }

        match grid[nj as usize][ni as usize] {
            '#' => {
                position = rotate(&position);
            }
            '.' => {
                position = walk(&position);

                counter += 1;

                grid[nj as usize][ni as usize] = 'X';
            }
            'X' => {
                position = walk(&position);
            }

            _ => panic!("unknown tile type {}", grid[nj as usize][ni as usize]),
        }
    }

    println!("{}", counter);

    ()
}

fn rotate((orientation, j, i): &(char, i64, i64)) -> (char, i64, i64) {
    match orientation {
        '^' => ('>', *j, *i),
        '>' => ('v', *j, *i),
        'v' => ('<', *j, *i),
        '<' => ('^', *j, *i),
        _ => panic!("not found orientation {}", orientation),
    }
}

fn walk((orientation, j, i): &(char, i64, i64)) -> (char, i64, i64) {
    match orientation {
        'v' => ('v', j + 1, *i),
        '>' => ('>', *j, i + 1),
        '<' => ('<', *j, i - 1),
        '^' => ('^', j - 1, *i),
        _ => panic!("not found orientation {}", orientation),
    }
}

fn next((orientation, j, i): &(char, i64, i64)) -> (i64, i64) {
    match orientation {
        'v' => (j + 1, *i),
        '>' => (*j, i + 1),
        '<' => (*j, i - 1),
        '^' => (j - 1, *i),
        _ => panic!("not found orientation {}", orientation),
    }
}
