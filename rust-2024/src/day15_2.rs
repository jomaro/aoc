use std::{collections::VecDeque, io::{self, BufRead}};

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

    // println!("{:?}", grid);
    // println!("{:?}", moves);

    // let mut robot = find_robot(&grid);

    // print_grid(&grid, robot);

    grid = expand_grid(&grid);

    let mut robot = find_robot(&grid);

    // print_grid(&grid, robot);

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

fn walk(grid: &mut Vec<Vec<char>>, p_src: (usize, usize), p_dst: (usize, usize)) {
    grid[p_dst.0][p_dst.1] = grid[p_src.0][p_src.1];
    grid[p_src.0][p_src.1] = '.';
}

fn make_move_raw(position: (usize, usize), move_: char, grid: &mut Vec<Vec<char>>) {
    let next = next_pos(position, move_);
    walk(grid, position, next);
}

fn make_move(robot: (usize, usize), move_: char, grid: &mut Vec<Vec<char>>) -> (usize, usize) {
    let next = next_pos(robot, move_);

    match grid[next.0][next.1] {
        '.' => {
            walk(grid, robot, next);

            next
        }
        '[' | ']' => {
            if move_ == '<' || move_ == '>' {
                make_move(next, move_, grid);

                if grid[next.0][next.1] == '.' {
                    make_move(robot, move_, grid)
                } else {
                    robot
                }
            } else {
                let boxes = make_graph(robot, move_, &grid);

                // println!("{:?}", boxes);

                for position in &boxes {
                    let (j, i) = next_pos(*position, move_);

                    if grid[j][i] == '#' {
                        return robot;
                    }
                }

                for position in &boxes {
                    make_move_raw(*position, move_, grid);
                }

                next
            }
        }
        '#' => robot,
        tile => {
            panic!("Unknown tile type {tile}");
        }
    }
}

fn make_graph(position: (usize, usize), move_: char, grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();

    points.push(position);

    let next = next_pos(position, move_);

    queue.push_back(next);

    while queue.len() > 0 {
        let position = queue.pop_front().unwrap();

        make_graph_rec(position, move_, grid, &mut points, &mut queue);
    }

    points.reverse();

    points
}

fn make_graph_rec(
    position: (usize, usize),
    move_: char,
    grid: &Vec<Vec<char>>,
    points: &mut Vec<(usize, usize)>,
    queue: &mut VecDeque<(usize, usize)>,
) {
    if let Some(p) = expand_box(position, grid) {
        add_new(position, points);
        add_new(p, points);

        add_new_deque(next_pos(position, move_), queue);
        add_new_deque(next_pos(p, move_), queue);
    }
}

fn add_new(position: (usize, usize), points: &mut Vec<(usize, usize)>) {
    if !points.contains(&position) {
        points.push(position);
    }
}

fn add_new_deque(position: (usize, usize), points: &mut VecDeque<(usize, usize)>) {
    if !points.contains(&position) {
        points.push_back(position);
    }
}

fn expand_box((j, i): (usize, usize), grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    if grid[j][i] == '[' {
        return Some((j, i + 1));
    }
    if grid[j][i] == ']' {
        return Some((j, i - 1));
    }

    return None;
}

fn next_pos((j, i): (usize, usize), move_: char) -> (usize, usize) {
    match move_ {
        '>' => (j, i + 1),
        '<' => (j, i - 1),
        '^' => (j - 1, i),
        'v' => (j + 1, i),
        m => panic!("Unknown move `{m}`"),
    }
}

fn expand_grid(grid: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new: Vec<Vec<char>> = Vec::new();

    for line in grid {
        let mut nline: Vec<char> = Vec::new();

        for v in line {
            let n = match v {
                '@' => vec!['@', '.'],
                '.' => vec!['.', '.'],
                '#' => vec!['#', '#'],
                'O' => vec!['[', ']'],
                tile => panic!("Unknown tile type {tile}"),
            };

            nline.extend(n);
        }

        new.push(nline);
    }

    new
}

fn find_robot(grid: &Vec<Vec<char>>) -> (usize, usize) {
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
            if grid[j][i] == '[' {
                sum += 100 * j + i;
            }
        }
    }

    sum
}
