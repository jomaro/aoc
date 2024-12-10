use std::io::{self, BufRead};

pub fn solve() -> () {
    let line: Vec<i64> = io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .unwrap();

    // println!("{:?}", line);

    // println!("{:?}", line.iter().sum::<i64>());

    let size = line.iter().sum::<i64>();

    let mut file: Vec<Option<i64>> = Vec::with_capacity(size as usize);

    let mut flip = true;
    let mut file_count: i64 = 0;

    for i in line {
        if flip {
            for _ in 0..i {
                file.push(Some(file_count));
            }

            file_count += 1;
        } else {
            for _ in 0..i {
                file.push(None);
            }
        }

        flip = !flip;
    }

    // println!("{:?}", file);

    let mut i = 0usize;

    while i < file.len() {
        if file[i].is_none() {
            let mut last = None;

            while last.is_none() {
                last = file.pop().unwrap()
            }

            file[i] = last;
        }

        i += 1;
    }

    // println!("{:?}", file);
    // println!("{:?}", file.iter().map(|c| c.unwrap().to_string()).collect::<Vec<String>>().join(""));

    let checksum: i64 = file
        .iter()
        .enumerate()
        .map(|(index, file)| index as i64 * file.unwrap())
        .sum();

    println!("{}", checksum);

    ()
}
