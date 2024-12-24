use std::io::{self, BufRead};
use regex::Regex;

enum OpCode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7
}

impl OpCode {

    pub fn from_code(e: i64) -> Self {
        match e {
            0 => Self::Adv,
            1 => Self::Bxl,
            2 => Self::Bst,
            3 => Self::Jnz,
            4 => Self::Bxc,
            5 => Self::Out,
            6 => Self::Bdv,
            7 => Self::Cdv,
            o => panic!("Unknown {o}"),
        }
    } 
}

pub fn solve() -> () {
    let lines: Vec<String> =
        io::stdin()
            .lock()
            .lines()
            .map(|line| line.unwrap())
            .collect();


    let ra = Regex::new(r"Register (\w+): (\d+)").unwrap();
    let mut reg_a = extract(&ra, &lines[0]);
    let mut reg_b = extract(&ra, &lines[1]);
    let mut reg_c = extract(&ra, &lines[2]);

    let ops: Vec<i64> = lines[4].split(" ").nth(1).unwrap().split(",").map(|op| op.parse::<i64>().unwrap()).collect();

    let mut pc = 0i64;
    let limit = ops.len() as i64;

    let mut outputs: Vec<i64> = Vec::new();

    while pc < limit {
        let op = ops[pc as usize];
        let reg = ops[pc as usize +1];

        match OpCode::from_code(op) {
            OpCode::Adv => {
                reg_a = reg_a / 2i64.pow(combo(reg, reg_a, reg_b, reg_c) as u32);

                pc += 2;
            },
            OpCode::Bxl => {
                reg_b = reg_b ^ reg;
                pc += 2;
            },
            OpCode::Bst => {
                reg_b = combo(reg, reg_a, reg_b, reg_c) & 7;
                pc += 2;
            },
            OpCode::Jnz => {
                if reg_a == 0 {
                    pc += 2;
                } else {
                    pc = reg;
                }
            },
            OpCode::Bxc => {
                reg_b = reg_b ^ reg_c;
                pc += 2;
            },
            OpCode::Out => {
                outputs.push(combo(reg, reg_a, reg_b, reg_c) & 7);
                pc += 2;
            },
            OpCode::Bdv => {
                reg_b = reg_a / 2i64.pow(combo(reg, reg_a, reg_b, reg_c) as u32);

                pc += 2;
            },
            OpCode::Cdv => {
                reg_c = reg_a / 2i64.pow(combo(reg, reg_a, reg_b, reg_c) as u32);

                pc += 2;
            }
        }
    }

    println!("{:?}", outputs.iter().map(|a| a.to_string()).collect::<Vec<String>>().join(","));

    ()
}

fn combo(reg: i64, reg_a: i64, reg_b: i64, reg_c: i64) -> i64 {
    match reg {
        0 | 1 | 2 | 3 => reg,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        o => panic!("invalid combo operand {o}")
    }
}

fn extract(r: &Regex, s: &String) -> i64 {
    r.captures_iter(s)
        .next()
        .map(|c| c.extract())
        .map(|(_, [_a, b])| {
            b.parse::<i64>().unwrap()
        })
        .unwrap()
}
