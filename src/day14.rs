use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;

lazy_static! {
  static ref MEMSET_REGEX: Regex = Regex::new(r"^mem\[(\d+)\]\s=\s(\d+)$").unwrap();
}

#[derive(Debug)]
enum Operation<'a> {
  Mask(&'a str),
  MemSet((i64, i64)),
}

fn parse_operation<'a>(input: &'a str) -> Operation<'a> {
  // ^mem\[(\d+)\]\s=\s(\d+)$
  if input.starts_with("mask") {
    let mask: &str = input.split(' ').nth(2).unwrap();
    return Operation::Mask(mask);
  }
  let captures = MEMSET_REGEX.captures(input).unwrap();
  let address: i64 = captures
    .get(1)
    .map(|x| x.as_str().parse().unwrap())
    .unwrap();
  let value: i64 = captures
    .get(2)
    .map(|x| x.as_str().parse().unwrap())
    .unwrap();
  Operation::MemSet((address, value))
}

fn apply_mask(mask: &str, value: i64) -> i64 {
  let len = 35;
  let mut output = value;
  for (i, bit) in mask.chars().enumerate() {
    match bit {
      '1' => output = output | (1 << len - i),
      '0' => output = output & !(1 << len - i),
      _ => continue,
    }
  }
  output
}

fn run_part_1(input: &str) -> i64 {
  let mut memory: HashMap<i64, i64> = HashMap::new();
  let mut mask: &str = "";
  for line in input.lines() {
    let operation = parse_operation(line);
    if let Operation::Mask(val) = operation {
      mask = val;
    } else if let Operation::MemSet((address, value)) = operation {
      memory.insert(address, apply_mask(mask, value));
    }
  }
  memory.into_iter().fold(0, |a, (_, v)| v + a)
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(run_part_1(input))
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT_EXAMPLE_1: &str = r"mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

  #[test]
  fn test_part1() {
    assert_eq!(165, run_part_1(INPUT_EXAMPLE_1));
  }
}
