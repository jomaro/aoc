
use std::io::{self, BufRead};
use std::collections::{HashMap, HashSet};


#[derive(Debug, Clone)]
struct Position(char, i64, i64);


pub fn solve() -> () {
  let mut grid : Vec<Vec<char>> = io::stdin()
      .lock()
      .lines()
      .map(|line| line.unwrap().chars().collect::<Vec<char>>())
      .collect();

  let y = grid.len() as i64;
  let x = grid[0].len() as i64;

  let mut counter = 0;
  let mut steps = 0;

  let guard: Vec<char> = "v<>^".chars().collect();

  let mut position: Option<Position> = None;

  for j in 0..y {
    for i in 0..x {
      if guard.contains(&grid[j as usize][i as usize]) {
        position = Some(Position(grid[j as usize][i as usize], j, i));

        grid[j as usize][i as usize] = '.';
      }
    }
  }

  let mut position: Position = position.unwrap();

  // println!("{:?}", grid);

  loop {
    let (nj, ni) = next(&position);

    if nj < 0 || nj >= y || ni < 0 || ni >= x {
      break;
    }

    match grid[nj as usize][ni as usize] {
      '#' => {
        position = rotate(&position);

        println!("> {} {:?}", steps, position);
      },
      '.' => {
        position = walk(&position);

        println!("= {} {:?}", steps, position);

        steps += 1;

        let (j, i) = next(&position);

        if j < 0 || j >= y || i < 0 || i >= x {
          continue;
        }

        let old = grid[j as usize][i as usize];

        grid[j as usize][i as usize] = '#';

        counter += run(&position, y, x, &grid);

        grid[position.1 as usize][position.2 as usize] = old;
      },
      tile => panic!("unknown tile type {}", tile)
    }
  }

  println!("{}", counter);

  ()
}

fn run(position: &Position, y: i64, x: i64, grid: &Vec<Vec<char>>) -> i64 {
  let mut position: Position = position.clone();

  let mut visited: HashMap<(i64, i64), HashSet<char>> = HashMap::new();

  visited_insert(&mut visited, &position);

  loop {
    let (nj, ni) = next(&position);

    if nj < 0 || nj >= y || ni < 0 || ni >= x {
      return 0;
    }

    match grid[nj as usize][ni as usize] {
      '#' => {
        position = rotate(&position);

        visited_insert(&mut visited, &position);
      },
      '.' => {
        position = walk(&position);

        if visited.contains_key(&(position.1, position.2)) {
          if visited.get(&(position.1, position.2)).unwrap().contains(&position.0) {
            return 1;
          }
        }

        visited_insert(&mut visited, &position);
      },
      tile => panic!("unknown tile type {}", tile)
    }
  }
}

fn visited_insert(visited : &mut HashMap<(i64, i64), HashSet<char>>, position: &Position)  {
  let v = {
    let mut v = HashSet::new();

    v.insert(position.0);

    v
  };

  visited
    .entry((position.1, position.2))
    .and_modify(|entry| { entry.insert(position.0); })
    .or_insert(v);
}

fn rotate(position: &Position) -> Position {
  Position(orientation_next(position.0), position.1, position.2)
}

fn walk(position: &Position) -> Position {
  match position.0 {
    'v' => Position('v', position.1+1, position.2),
    '>' => Position('>', position.1, position.2+1),
    '<' => Position('<', position.1, position.2-1),
    '^' => Position('^', position.1-1, position.2),
    _   => panic!("not found orientation {}", position.0)
  }
}

fn orientation_next(orientation: char) -> char {
  match orientation {
    '>' => 'v',
    'v' => '<',
    '<' => '^',
    '^' => '>',
    _   => panic!("not found orientation {}", orientation)
  }
}

fn next(position: &Position) -> (i64, i64) {
  match position.0 {
    'v' => (position.1+1, position.2),
    '>' => (position.1, position.2+1),
    '<' => (position.1, position.2-1),
    '^' => (position.1-1, position.2),
    _   => panic!("not found orientation {}", position.0)
  }
}
