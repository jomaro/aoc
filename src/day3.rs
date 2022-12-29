

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;

pub fn solve_part1() {
    let mut buffer = String::new();
    let mut handle = io::stdin().lock();

    let mut sum: u64 = 0;

    while handle.read_line(&mut buffer).unwrap() != 0 {
        let line = buffer.trim();

        if line.is_empty() {
            continue;
        }

        sum += solve_line_for_1(line.as_bytes()) as u64;

        buffer.clear();
    }

    println!("{}", sum);
}

fn solve_line_for_1(line: &[u8]) -> u8 {
    let length = line.len();

    let mut alphabet = [0u8; 256];

    let (first, second) = line.split_at(length / 2);

    for letter in first {
        alphabet[*letter as usize] = 1;
    }

    for letter in second {
        if alphabet[*letter as usize] != 0 {
            return get_priority_for_letter(*letter);
        }
    }

    panic!("Did not found a repeated letter");
}

fn get_priority_for_letter(letter: u8) -> u8 {
    if letter >= b'a' && letter <= b'z' {
        return letter - b'a' + 1;
    }

    return letter - b'A' + 27;
}

pub fn solve_part2() {
    let file: Vec<String> = env::args().collect();
    let file = file[2].to_owned();

    let result: u64 = read_lines(file).unwrap().map(|result| result.unwrap()).array_chunks::<3>().map(solve_for_part_2).sum();

    println!("{}", result);
}

fn solve_for_part_2(args: [String; 3]) -> u64 {
    let [l1, l2, l3] = args;

    let l1 = l1.as_bytes();
    let l2 = l2.as_bytes();
    let l3 = l3.as_bytes();
    let mut alphabet = [0u8; 256];

    for letter in l1 {
        let i = *letter as usize;
        alphabet[i] = alphabet[i] | 0x1;
    }

    for letter in l2 {
        let i = *letter as usize;
        alphabet[i] = alphabet[i] | 0x2;
    }

    for letter in l3 {
        if alphabet[*letter as usize] == 0x3 {
            return (get_priority_for_letter(*letter) as u64).try_into().unwrap();
        }
    }

    panic!("Should not get here");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
