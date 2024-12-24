use std::collections::HashMap;use std::collections::HashSet;
use std::io::{self, BufRead};

use super::utils::grid::{Coord, D2vec, Grid};

const MIN: i64 = 100;
// const MIN: i64 = 76;
// const MIN: i64 = 50;
const CHEAT_SIZE: i64 = 20;
// const CHEAT_SIZE: i64 = 6;

pub fn solve() -> () {
    let mut grid: Grid<char> = Grid(
        io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect(),
    );

    let start: Coord = locate_tile(&grid, 'S');
    let end: Coord = locate_tile(&grid, 'E');

    grid.set(&start, '.');
    grid.set(&end, '.');

    let mut pointer: Coord = start.clone();

    let mut path: Vec<Coord> = Vec::new();
    let mut path_map: HashMap<Coord, i64> = HashMap::new();
    let mut counter = 0i64;

    let navigation: Vec<D2vec> = vec![D2vec(0, 1), D2vec(0, -1), D2vec(1, 0), D2vec(-1, 0)];

    while pointer != end {
        path.push(pointer);
        path_map.insert(pointer, counter);

        counter += 1;

        pointer = pointer.add(
            *navigation
                .iter()
                .filter(|v| {
                    !path_map.contains_key(&pointer.add(**v)) && grid.get(&pointer.add(**v)) == '.'
                })
                .next()
                .unwrap(),
        );
    }
    
    path_map.insert(end, counter);

    path.push(end);

    // println!("{:?}", path.len());
    // println!("{:?}", path_map);
    // println!("{}", counter);

    let cheats: HashSet<D2vec> = make_vectors_up_to_size(CHEAT_SIZE);

    // println!("{:?}", cheats);

    let mut counter: i64 = 0;

    let mut dists: HashMap<i64, i64> = HashMap::new();

    for (point, i) in &path_map {
        for c in &cheats {
            if let Some(j) = path_map.get(&point.add(*c)) {
                let dist = i - j - (*c).size();

                if dist >= MIN {
                    dists.entry(dist).and_modify(|e| *e += 1).or_insert(1);
                    // println!("{:?} {} {} {}", point, i, j, i-j);
                    counter += 1;
                }
            }
        }
    }

    // println!("{:?}", dists);

    println!("{}", counter);

    ()
}

fn make_vectors_up_to_size(n: i64) -> HashSet<D2vec> {
    let mut v: HashSet<D2vec> = HashSet::new();

    for i in 0..=n {
        for j in 0..=(n-i) {
            v.insert(D2vec(j, i));
            v.insert(D2vec(j, -i));
            v.insert(D2vec(-j, i));
            v.insert(D2vec(-j, -i));
        }
    }

    v
}

fn locate_tile(grid: &Grid<char>, tile: char) -> Coord {
    for j in 0..grid.y() {
        for i in 0..grid.x() {
            if grid.get(&(j, i)) == tile {
                return Coord(j as i64, i as i64);
            }
        }
    }

    panic!("Could not find robot");
}
