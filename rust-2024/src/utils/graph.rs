use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::collections::{HashSet, HashMap};

use std::cmp::Eq;
use std::hash::Hash;
use std::fmt::Debug;

const INFINITY: i64 = i64::max_value() >> 3;

#[derive(Debug, Clone)]
pub struct Graph<T>
where T: Copy + Clone + ToString + Eq + Hash + Debug{
  vertices: HashSet<T>,
  edges: HashMap<T, HashSet<T>>,
}

#[allow(dead_code)]
impl<T: Copy + Clone + ToString + Eq + Hash + Debug> Graph<T> {
  pub fn new() -> Self {
    Self{
      vertices: HashSet::new(),
      edges: HashMap::new(),
    }
  }

  pub fn add_vertice(&mut self, v: T) {
    self.vertices.insert(v);
  }

  pub fn remove_vertice(&mut self, v: T) {
    self.vertices.remove(&v);
  }

  pub fn add_edge(&mut self, v1: T, v2: T) {
    self.edges.entry(v1).or_default().insert(v2);
    self.edges.entry(v2).or_default().insert(v1);
  }

  pub fn remove_edge(&mut self, v1: T, v2: T) {
    self.edges.entry(v1).or_default().remove(&v2);
    self.edges.entry(v2).or_default().remove(&v1);
  }

  pub fn dijkstra(&self, start: &T) -> (HashMap<T, i64>, HashMap<T, Option<T>>) {
    let mut dist: HashMap<T, i64> = HashMap::new();
    let mut prev: HashMap<T, Option<T>> = HashMap::new();

    let mut queue: PriorityQueue<T, Reverse<i64>> = PriorityQueue::new();

    for v in self.vertices.clone() {
        dist.insert(v, INFINITY);
        prev.insert(v, None);

        queue.push(v, Reverse(INFINITY));
    }

    dist.insert(*start, 0);

    queue.change_priority(&start, Reverse(0));

    while !queue.is_empty() {
        let (u, _) = queue.pop().unwrap();

        if let Some(neightbours) = self.edges.get(&u) {
            for v in neightbours {
                if queue.get_priority(v).is_none() {
                    continue;
                }

                let alt = dist.get(&u).unwrap() + 1;
                if alt < *dist.get(v).unwrap() {
                    dist.insert(*v, alt);
                    queue.change_priority(v, Reverse(alt));
                    prev.insert(*v, Some(u));
                }
            }
        }
    }

    (dist, prev)
  }
}
