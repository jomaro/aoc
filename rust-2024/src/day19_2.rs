use std::io::{self, BufRead};
use std::collections::HashMap;


pub fn solve() -> () {
    let mut l = io::stdin()
        .lock()
        .lines();

    let patterns: Vec<String> = l.next().unwrap().unwrap().split(", ").map(|s| s.to_string()).collect();

    let towels: Vec<String> = l
            .map(|line| line.unwrap())
            .filter(|line| !line.is_empty())
            .collect();

    

    let result: i64 = towels.iter().map(|towel| {
        println!("{}", towel);

        let mut cache: HashMap<usize, i64> = HashMap::new();

        dbg!(solve_towel(&towel, towel.len(), &patterns, &mut cache))
    }).sum();

    println!("{}", result);
    
    ()
}

fn solve_towel(towel: &str, len: usize, patterns: &Vec<String>, cache: &mut HashMap<usize, i64>) -> i64 {
    if towel.is_empty() {
        return 1;
    }

    if cache.contains_key(&len) {
        return *cache.get(&len).unwrap();
    }

    let mut counter: i64 = 0;

    for p in patterns {
        if towel.starts_with(p) {
            counter += solve_towel(&towel[p.len()..], len - p.len(), patterns, cache);
        }
    }

    cache.insert(len, counter);

    return counter;
}
