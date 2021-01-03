use std::error::Error;

struct Input {
  low: i64,
  high: i64,
  c: char,
  password: String,
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  let inputs: Vec<Input> = input.split("\n").map(|input| parse_input(input)).collect();
  let mut matches = 0;
  for i in inputs {
    let count = i.password.matches(i.c).collect::<Vec<&str>>().len() as i64;
    if count >= i.low && count <= i.high {
      matches += 1;
    }
  }
  Ok(matches)
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  let inputs: Vec<Input> = input.split("\n").map(|input| parse_input(input)).collect();
  let mut matches = 0;
  for i in inputs {
    let indices: Vec<_> = i.password.match_indices(i.c).collect();
    let mut inner_match = 0;
    for (loc, _) in indices {
      if loc == (i.low - 1) as usize || loc == (i.high - 1) as usize {
        inner_match += 1;
      }
    }
    if inner_match == 1 {
      matches += 1;
    }
  }
  Ok(matches)
}

fn parse_input(input: &str) -> Input {
  let parts: Vec<&str> = input.split(' ').collect();
  let low_high: Vec<i64> = parts[0]
    .split('-')
    .map(|val| val.parse().unwrap())
    .collect();
  let c: char = parts[1].chars().nth(0).unwrap();
  let password = String::from(parts[2]);

  Input {
    low: low_high[0],
    high: low_high[1],
    c,
    password,
  }
}
