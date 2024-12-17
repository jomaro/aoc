use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::io::{self, BufRead};

use super::utils::grid::{Grid, Orientation, Position};

const ROTATION_COST: i64 = 1_000;
const WALK_COST: i64 = 1;
const INFINITY: i64 = i64::max_value() >> 3;

pub fn solve() -> () {
    let grid: Grid<char> = Grid(
        io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap().chars().collect())
            .collect(),
    );

    // grid.print();

    let mut vertices: Vec<Position> = Vec::new();
    let mut edges: HashMap<Position, Vec<(Position, i64)>> = HashMap::new();

    for j in 0..grid.y() {
        for i in 0..grid.x() {
            if vec!['.', 'S', 'E'].contains(&grid.get(&(j, i))) {
                let mut p = Position(Orientation::Up, j, i);
                vertices.push(p);
                vertices.push(p.rotate_right());
                vertices.push(p.rotate_right().rotate_right());
                vertices.push(p.rotate_right().rotate_right().rotate_right());

                for _ in 0..4 {
                    edges.insert(
                        p,
                        vec![
                            (p.rotate_right(), ROTATION_COST),
                            (p.rotate_left(), ROTATION_COST),
                        ],
                    );

                    let next = p.walk();

                    if vec!['.', 'S', 'E'].contains(&grid.get(&next)) {
                        edges.entry(p).and_modify(|e| e.push((next, WALK_COST)));
                    }

                    p = p.rotate_right();
                }
            }
        }
    }

    let start: Position = locate_tile(&grid, 'S');
    let end: Position = locate_tile(&grid, 'E');
    let end: Vec<Position> = vec![
        end,
        end.rotate_right(),
        end.rotate_right().rotate_right(),
        end.rotate_left(),
    ];

    let (dist, _prev) = dijkstra(&vertices, &edges, &start);

    let result = end.iter().map(|p| dist.get(p).unwrap()).min().unwrap();

    println!("{}", result);

    ()
}

fn dijkstra(
    vertices: &Vec<Position>,
    edges: &HashMap<Position, Vec<(Position, i64)>>,
    start: &Position,
) -> (HashMap<Position, i64>, HashMap<Position, Option<Position>>) {
    let mut dist: HashMap<Position, i64> = HashMap::new();
    let mut prev: HashMap<Position, Option<Position>> = HashMap::new();

    let mut queue: PriorityQueue<Position, Reverse<i64>> = PriorityQueue::new();

    for v in vertices {
        dist.insert(*v, INFINITY);
        prev.insert(*v, None);

        queue.push(*v, Reverse(INFINITY));
    }

    dist.insert(*start, 0);

    queue.change_priority(&start, Reverse(0));

    while !queue.is_empty() {
        let (u, _) = queue.pop().unwrap();

        for (v, weight) in edges.get(&u).unwrap() {
            if queue.get_priority(v).is_none() {
                continue;
            }

            let alt = dist.get(&u).unwrap() + weight;
            if alt < *dist.get(v).unwrap() {
                dist.insert(*v, alt);
                queue.change_priority(v, Reverse(alt));
                prev.insert(*v, Some(u));
            }
        }
    }

    (dist, prev)
}

fn locate_tile(grid: &Grid<char>, tile: char) -> Position {
    for j in 0..grid.y() {
        for i in 0..grid.x() {
            if grid.get(&(j, i)) == tile {
                return Position(Orientation::Right, j as i64, i as i64);
            }
        }
    }

    panic!("Could not find robot");
}
