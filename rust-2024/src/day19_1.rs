use std::io::{self, BufRead};
use regex::Regex;


pub fn solve() -> () {
    let mut l = io::stdin()
        .lock()
        .lines();

    let patterns: Vec<String> = l.next().unwrap().unwrap().split(", ").map(|s| s.to_string()).collect();

    let towels: Vec<String> = l
            .map(|line| line.unwrap())
            .collect();

    let re =  Regex::new(&format!("^({})+$", patterns.join("|"))).unwrap();

    let result = towels.iter().filter(|towel| re.is_match(towel)).count();

    println!("{}", result);
    
    ()
}
