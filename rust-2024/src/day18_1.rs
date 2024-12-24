use std::io::{self, BufRead};
use std::collections::HashSet;

use super::utils::grid::{Grid, Coord};
use super::utils::graph::Graph;

// const LIMIT: i64 = 6;
// const FALLEN_BYTES: usize = 12;
const LIMIT: i64 = 70;
const FALLEN_BYTES: usize = 1_024;

pub fn solve() -> () {
    let holes: HashSet<(i64, i64)> =
        io::stdin()
            .lock()
            .lines()
            .take(FALLEN_BYTES)
            .map(|line| {
                let e = line.unwrap();

                let (a, b) = e.split_once(",").unwrap();

                (
                    b.parse().unwrap(),
                    a.parse().unwrap()
                )
            })
            .collect();

    let mut lines: Vec<Vec<char>>= Vec::new();

    for j in 0..=LIMIT {
        let mut line = Vec::new();

        for i in 0..=LIMIT {
            if holes.contains(&(j,i)) {
                line.push('#');
            } else {
                line.push('.');
            }
        }

        lines.push(line);
    }

    let g = Grid(lines);

    g.print();
    let mut gr: Graph<Coord> = Graph::new();

    for j in 0..=LIMIT {
        for i in 0..=LIMIT {
            if holes.contains(&(j,i)) {
                continue;
            }

            gr.add_vertice(Coord(j,i));

            let v = (j,i+1);
            if g.within_bounds(&v) && g.get(&v) != '#' {
                gr.add_edge(Coord(j,i), Coord::new(v));
            }

            let v = (j,i-1);
            if g.within_bounds(&v) && g.get(&v) != '#' {
                gr.add_edge(Coord(j,i), Coord::new(v));
            }

            let v = (j+1,i);
            if g.within_bounds(&v) && g.get(&v) != '#' {
                gr.add_edge(Coord(j,i), Coord::new(v));
            }

            let v = (j-1,i);
            if g.within_bounds(&v) && g.get(&v) != '#' {
                gr.add_edge(Coord(j,i), Coord::new(v));
            }
        }
    }

    // println!("{:?}", gr);

    let (dist, _prev) = gr.dijkstra(&Coord(0, 0));

    println!("{}", dist.get(&Coord(LIMIT, LIMIT)).unwrap());
    
    ()
}
