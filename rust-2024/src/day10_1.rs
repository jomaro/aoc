use std::io::{self, BufRead};
use std::collections::HashSet;

pub fn solve() -> () {
  let grid : Vec<Vec<u8>> = io::stdin()
      .lock()
      .lines()
      .map(|line| line.unwrap().chars().map(|c| c.to_string().parse::<u8>().unwrap()).collect::<Vec<u8>>())
      .collect();

  let y = grid.len() as i64;
  let x = grid[0].len() as i64;

  let mut trailheads: HashSet<(i64, i64, i64, i64)> = HashSet::new();
  
  for j in 0..y {
    for i in 0..x {
        search(&grid, y, x, j, i, 0u8, j, i, &mut trailheads);
      }
  }

  println!("{}", trailheads.len());

  ()
}

fn search(grid: &Vec<Vec<u8>>, y: i64, x: i64, j: i64, i: i64, position: u8, sj: i64, si: i64, trailheads: &mut HashSet<(i64, i64, i64, i64)>) {
  if j < 0 || j >= y || i < 0 || i >= x {
    return;
  }

  if grid[j as usize][i as usize] != position {
    return;
  }

  if position == 9 {
    trailheads.insert((j,i,sj,si));
  }

  search(grid, y, x, j-1, i, position + 1, sj, si, trailheads);
  search(grid, y, x, j+1, i, position + 1, sj, si, trailheads);
  search(grid, y, x, j, i-1, position + 1, sj, si, trailheads);
  search(grid, y, x, j, i+1, position + 1, sj, si, trailheads);

}
