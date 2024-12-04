
use std::io::{self, BufRead};
use regex::Regex;


pub fn solve() -> () {
  let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\))").unwrap();

  let mut enabled = true;
  let mut value = 0i64;

  for line in io::stdin().lock().lines() {
    let cline = line.unwrap().clone();

    for capture in re.captures_iter(&cline.as_str()) {
      match capture.get(0).unwrap().as_str() {
        "do()" => {
          enabled = true;
        }

        "don't()" => {
          enabled = false;
        }

        _ => {
          if enabled {
            let a = capture.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let b = capture.get(3).unwrap().as_str().parse::<i64>().unwrap();
    
            println!("{:?} {:?} {:?}", a, b, a * b);
    
            value += a * b;
          }
        }
      }
    }
  }

  println!("{}", value);

  ()
}
