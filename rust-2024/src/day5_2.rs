use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, BufRead};

pub fn solve() -> () {
    let (rules, updates) = parse();

    let value: i64 = updates
        .iter()
        .filter(|update| !is_ordered(update, &rules))
        .map(|update| {
            let sorted = topological_sort(update, &rules);

            let v = sorted[sorted.len() / 2];

            println!("{}", v);

            v
        })
        .sum();

    println!("{}", value);

    ()
}

fn topological_sort(update: &Vec<i64>, rules: &Vec<(i64, i64)>) -> Vec<i64> {
    let mut graph: HashMap<i64, Vec<i64>> = HashMap::new();

    for v in update.iter() {
        graph.insert(*v, vec![]);
    }

    for (n1, n2) in rules {
        if graph.get(n1).is_some() && graph.get(n2).is_some() {
            graph.entry(*n1).and_modify(|entry| (*entry).push(*n2));
        }
    }

    println!("{:?}", graph);
    println!("{:?}", graph.len());

    let mut stack: Vec<i64> = vec![];

    let mut visited: HashSet<i64> = HashSet::new();

    for v in update {
        rec(*v, &graph, &mut visited, &mut stack);
    }

    stack.reverse();

    stack
}

fn rec(
    index: i64,
    graph: &HashMap<i64, Vec<i64>>,
    visited: &mut HashSet<i64>,
    stack: &mut Vec<i64>,
) {
    if let Some(_) = visited.get(&index) {
        return;
    }

    visited.insert(index);

    for v in graph.get(&index).unwrap() {
        rec(*v, &graph, visited, stack)
    }

    stack.push(index);
}

fn is_ordered(update: &&Vec<i64>, rules: &Vec<(i64, i64)>) -> bool {
    let order: HashMap<i64, i64> = update
        .into_iter()
        .enumerate()
        .map(|(index, value)| (value.clone(), index as i64))
        .collect();

    rules
        .iter()
        .all(|(n1, n2)| match (order.get(&n1), order.get(&n2)) {
            (None, _) => true,
            (_, None) => true,
            (Some(n1), Some(n2)) => n1 < n2,
        })
}

fn parse() -> (Vec<(i64, i64)>, Vec<Vec<i64>>) {
    let mut finished_rules = false;

    let mut rules: Vec<(i64, i64)> = vec![];

    let mut updates: Vec<Vec<i64>> = vec![];

    for line in io::stdin().lock().lines() {
        let line = line.unwrap();

        if line == "" {
            finished_rules = true;
            continue;
        }

        if !finished_rules {
            let rule: Vec<i64> = line
                .split("|")
                .map(|number| number.parse::<i64>().unwrap())
                .collect();

            rules.push((rule[0], rule[1]));
        } else {
            let update: Vec<i64> = line
                .split(",")
                .map(|number| number.parse::<i64>().unwrap())
                .collect();

            updates.push(update);
        }
    }

    println!("{:?}", rules);
    println!("{:?}", updates);

    (rules, updates)
}
