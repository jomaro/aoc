use std::collections::HashMap;
use std::io::{self, BufRead};

pub fn solve() -> () {
    let (rules, updates) = parse();

    let value: i64 = updates
        .iter()
        .filter(|update| is_ordered(update, &rules))
        .map(|update| {
            let v = update[update.len() / 2];

            println!("{}", v);

            v
        })
        .sum();

    println!("{}", value);

    ()
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
