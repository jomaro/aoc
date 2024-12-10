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

    let mut file: Vec<(i64, i64, Option<i64>)> = Vec::new();

    let mut flip = true;
    let mut file_count: i64 = 0;
    let mut size_sum: i64 = 0;

    for i in line {
        if flip {
            file.push((size_sum, i, Some(file_count)));

            file_count += 1;
        } else {
            file.push((size_sum, i, None));
        }

        size_sum += i;

        flip = !flip;
    }

    // println!("{:?}", file);

    let mut i = file.len() - 1;

    loop {
        let block = file[i].clone();

        if block.2.is_none() {
            file.remove(i);
        } else {
            for j in 0..i {
                let (start, size, content) = file[j];

                if content.is_none() && size >= block.1 {
                    file.remove(i);
                    file.remove(j);
                    file.insert(j, (start, block.1, block.2));

                    if size > block.1 {
                        file.insert(j + 1, (start + block.1, size - block.1, None));

                        i += 1;
                    }

                    break;
                }
            }
        }

        if i == 0 {
            break;
        }

        i -= 1;
    }

    // println!("{:?}", file);
    // println!("{:?}", file.iter().map(|c| c.unwrap().to_string()).collect::<Vec<String>>().join(""));

    let checksum: i64 = file
        .iter()
        .map(|(start, size, content)| {
            if let Some(content) = content {
                ((0..*size).into_iter().sum::<i64>() + size * start) * content
            } else {
                0i64
            }
        })
        .sum();

    println!("{}", checksum);

    ()
}
