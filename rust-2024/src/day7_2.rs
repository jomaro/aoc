use std::io::{self, BufRead};

pub fn solve() -> () {
    let grid: Vec<(i64, Vec<i64>)> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let l = line.unwrap().clone();

            let (result, rest) = l.split_once(": ").unwrap();

            (
                result.parse::<i64>().unwrap(),
                rest.split(" ").map(|n| n.parse::<i64>().unwrap()).collect(),
            )
        })
        .collect();

    let mut counter = 0;

    for (result, numbers) in grid {
        let size = numbers.len() as i64;

        if rec(&numbers, 0, size, 0, result) {
            counter += result;
        }
    }

    println!("{}", counter);
}

fn rec(numbers: &Vec<i64>, index: i64, size: i64, acc: i64, result: i64) -> bool {
    if result < acc {
        return false;
    }

    if index == size && result == acc {
        return true;
    }

    if index == size {
        return false;
    }

    return rec(
        numbers,
        index + 1,
        size,
        acc + numbers[index as usize],
        result,
    ) || rec(
        numbers,
        index + 1,
        size,
        acc * numbers[index as usize],
        result,
    ) || rec(
        numbers,
        index + 1,
        size,
        my_concat(acc, numbers[index as usize]),
        result,
    );
}

fn my_concat(a: i64, b: i64) -> i64 {
    let size = b.to_string().len() as u32;

    a * 10i64.pow(size) + b
}
