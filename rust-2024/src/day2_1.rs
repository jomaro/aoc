
use std::io::{self, BufRead};


pub fn solve() -> () {
    let value : i64 = io::stdin().lock().lines()
        .map(|line| {
          let numbers : Vec<i64> =
            line
              .unwrap()
              .split(" ")
              .map(|number| number.parse::<i64>().unwrap())
              .collect();

          match verify_safety(&numbers) {
            Ok(_) => 1,
            Err(_) => 0
          }
        })
        .sum();

    println!("{}", value);

    ()
}

fn verify_safety(numbers: &Vec<i64>) -> Result<i64, i64> {
  let mut previous = numbers[0];
  let mut previous_polarity = polarity(numbers[0], numbers[1]);

  for n in numbers.iter().skip(1) {
    let n = *n;

    if n == previous {
      return Err(0);
    }

    if (n - previous).abs() > 3 {
      return Err(0);
    }

    let polarity = polarity(previous, n);

    if polarity != previous_polarity {
      return Err(0);
    }

    previous_polarity = polarity;
    previous = n;
  }

  Ok(1)
}

fn polarity(a: i64, b: i64) -> i64 {
  let n = b - a;

  if n < 0 {
    -1
  } else {
    1
  }
}
