use std::{
    collections::HashMap,
    io::{self, BufRead},
};

pub fn solve() -> () {
    let line: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|line| {
            line.split(" ")
                .map(|c| c.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .unwrap();

    let iterations = 75;

    let mut ctr: HashMap<(i64, i64), i64> = HashMap::new();

    let mut counter = 0i64;

    for item in line {
        counter += expand(item, iterations, &mut ctr);
    }

    println!("{}", counter);

    ()
}

fn expand(item: i64, level: i64, ctr: &mut HashMap<(i64, i64), i64>) -> i64 {
    if let Some(val) = ctr.get(&(item, level)) {
        return *val;
    }

    if level == 0 {
        return 1;
    }

    if item == 0 {
        let r = expand(1, level - 1, ctr);

        ctr.insert((item, level), r);

        return r;
    }

    let len = item.to_string().len() >> 1;

    if item.to_string().len() & 1 == 0 {
        let r = expand(
            item.to_string()[0..len].parse::<i64>().unwrap(),
            level - 1,
            ctr,
        ) + expand(
            item.to_string()[len..].parse::<i64>().unwrap(),
            level - 1,
            ctr,
        );

        ctr.insert((item, level), r);

        return r;
    }

    let r = expand(item * 2024, level - 1, ctr);

    ctr.insert((item, level), r);

    return r;
}
