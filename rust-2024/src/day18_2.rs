use std::io::{self, BufRead};

use super::utils::grid::{Grid, Coord};
use super::utils::graph::Graph;

// const LIMIT: i64 = 6;
const LIMIT: i64 = 70;
const INFINITY: i64 = i64::max_value() >> 3;

pub fn solve() -> () {
    let holes: Vec<(i64, i64)> =
        io::stdin()
            .lock()
            .lines()
            .map(|line| {
                let e = line.unwrap();

                let (i, j) = e.split_once(",").unwrap();

                (
                    j.parse().unwrap(),
                    i.parse().unwrap()
                )
            })
            .collect();

    let mut lines: Vec<Vec<char>>= Vec::new();

    for _ in 0..=LIMIT {
        let mut line = Vec::new();

        for _ in 0..=LIMIT {
            line.push('.');
        }

        lines.push(line);
    }

    let mut g = Grid(lines);

    let mut gr: Graph<Coord> = Graph::new();

    for j in 0..=LIMIT {
        for i in 0..=LIMIT {
            gr.add_vertice(Coord(j,i));

            let v = (j,i+1);
            if g.within_bounds(&v){
                gr.add_edge(Coord(j,i), Coord::new(v));
            }

            let v = (j,i-1);
            if g.within_bounds(&v){
                gr.add_edge(Coord(j,i), Coord::new(v));
            }

            let v = (j+1,i);
            if g.within_bounds(&v){
                gr.add_edge(Coord(j,i), Coord::new(v));
            }

            let v = (j-1,i);
            if g.within_bounds(&v){
                gr.add_edge(Coord(j,i), Coord::new(v));
            }
        }
    }

    for (_e, (j, i)) in holes.iter().enumerate() {
        let (j, i) = (*j, *i);

        g.set(&(j, i), '#');

        // println!("[[{}], {},{}]", e, j, i);
        // g.print();

        let c = Coord(j,i);

        gr.remove_vertice(c);

        let v = (j,i+1);
        gr.remove_edge(c, Coord::new(v));

        let v = (j,i-1);
        gr.remove_edge(c, Coord::new(v));

        let v = (j+1,i);
        gr.remove_edge(c, Coord::new(v));

        let v = (j-1,i);
        gr.remove_edge(c, Coord::new(v));

        let (dist, _prev) = gr.dijkstra(&Coord(0, 0));

        // println!("{:?}", dist.get(&Coord(LIMIT, LIMIT)));

        let d = dist.get(&Coord(LIMIT, LIMIT));

        if d.is_none() || *d.unwrap() == INFINITY {
            g.print();

            // we gotta print in the inverse order here, because it's the order the problem uses
            println!("{},{}", i, j);

            break;
        }
    }

    ()
}
