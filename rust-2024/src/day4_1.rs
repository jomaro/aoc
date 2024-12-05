
use std::io::{self, BufRead};

pub fn solve() -> () {
  let grid : Vec<Vec<char>> = io::stdin()
      .lock()
      .lines()
      .map(|line| line.unwrap().chars().collect::<Vec<char>>())
      .collect();

  let y = grid.len() as i64;
  let x = grid[0].len() as i64;

  let word : Vec<char> = "XMAS".chars().collect();

  let result = solve_cross(&grid, x, y, &word);

  println!("{}", result);

  ()
}

fn solve_cross(grid : &Vec<Vec<char>>, w: i64, h: i64, word: &Vec<char>) -> i64 {
  let vectors : Vec<(i64, i64)> = vec![(1,0), (1,1), (0,1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];

  let len = word.len() as i64;

  let mut counter = 0i64;

  for j in 0..h {
    for i in 0..w {
      for (ki, kj) in &vectors {
        let max_i = i+(len-1)*ki;
        let max_j = j+(len-1)*kj;

        if max_i < 0 || max_i >= w || max_j < 0 || max_j >= h {
          continue;
        } 

        let mut valid = true;

        for k in 0..len {
          if word[k as usize] != grid[(j+k*kj) as usize][(i+k*ki) as usize] {
            valid = false;
            break;
          }
        }

        if valid {
          counter += 1;
        }
      }
    }
  }

  counter
}
