use std::io::{self, BufRead};

pub fn solve() -> () {
    let mut grid: Vec<Vec<char>> = io::stdin()
        .lock()
        .lines()
        .take_while(|line| !line.as_ref().unwrap().is_empty())
        .map(|line| line.unwrap().chars().collect())
        .collect();

    let moves: Vec<char> = io::stdin()
        .lock()
        .lines()
        .flat_map(|line| line.unwrap().chars().collect::<Vec<char>>())
        .collect();

    println!("{:?}", grid);
    println!("{:?}", moves);

    let mut robot = robot(&grid);

    grid[robot.0][robot.1] = '.';

    for m in moves {
        robot = make_move(robot, m, &mut grid);
    }

    print_grid(&grid, robot);

    let result = count_gps(&grid);

    println!("{}", result);

    ()
}

fn print_grid(grid: &Vec<Vec<char>>, robot: (usize, usize)) {
    let mut grid = grid.clone();

    grid[robot.0][robot.1] = '@';

    for line in grid {
        println!("{}", line.iter().cloned().collect::<String>());
    }
}

fn make_move(robot: (usize, usize), move_: char, grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    let next = move_pos(robot, move_);

    match grid[next.0][next.1] {
        '.' => {
            grid[next.0][next.1] = grid[robot.0][robot.1];
            grid[robot.0][robot.1] = '.';

            next
        }
        'O' => {
            // println!("mov rec {:?} {:?} {:?}", robot, grid[robot.0][robot.1], grid[next.0][next.1]);
            make_move(next, move_, grid);

            if grid[next.0][next.1] == '.' {
                // println!("haha");
                make_move(robot, move_, grid)
            } else {
                robot
            }
        }
        '#' => robot,
        tile => {
            panic!("Unknown tile type {tile}");
        }
    }
}

fn move_pos((j, i): (usize, usize), move_: char) -> (usize, usize) {
    match move_ {
        '>' => (j, i + 1),
        '<' => (j, i - 1),
        '^' => (j - 1, i),
        'v' => (j + 1, i),
        m => panic!("Unknown move `{m}`"),
    }
}

fn robot(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let y = grid.len();
    let x = grid[0].len();

    for j in 0..y {
        for i in 0..x {
            if grid[j][i] == '@' {
                return (j, i);
            }
        }
    }

    panic!("Could not find robot");
}

fn count_gps(grid: &Vec<Vec<char>>) -> usize {
    let y = grid.len();
    let x = grid[0].len();

    let mut sum = 0;

    for j in 0..y {
        for i in 0..x {
            if grid[j][i] == 'O' {
                sum += 100 * j + i;
            }
        }
    }

    sum
}
