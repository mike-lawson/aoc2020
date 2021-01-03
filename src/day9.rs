use std::error::Error;

fn can_add(summers: &[i64], total: i64) -> bool {
  for i in 0..summers.len() {
    for ii in i + 1..summers.len() {
      if summers[i] + summers[ii] == total {
        return true;
      }
    }
  }
  false
}

fn can_contiguously_add(summers: &[i64], total: i64) -> (usize, usize) {
  for i in 0..summers.len() {
    let mut running_total = summers[i];
    let mut cursor = i + 1;
    while running_total < total {
      running_total += summers[cursor];
      cursor += 1;
    }
    if running_total == total {
      return (i, cursor);
    }
  }

  panic!("Not found!");
}

fn add_smallest_largest(range: &[i64]) -> i64 {
  let mut smallest = range[0];
  let mut largest = range[0];
  for &num in range {
    if num > largest {
      largest = num;
    }
    if num < smallest {
      smallest = num;
    }
  }
  largest + smallest
}

fn parse_part_2(input: &str, target: i64) -> i64 {
  let numbers: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
  let (start, end) = can_contiguously_add(&numbers, target);
  add_smallest_largest(&numbers[start..end])
}

fn parse_part_1(input: &str, pre: usize) -> i64 {
  let numbers: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();

  for i in pre..numbers.len() {
    let range_start = i - pre;
    let range_end = if pre + i > numbers.len() {
      numbers.len()
    } else {
      pre + i
    };

    if !can_add(&numbers[range_start..range_end], numbers[i]) {
      return numbers[i];
    }
  }

  panic!("Not found");
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_1(input, 25) as i64)
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_2(input, part1(input).unwrap()) as i64)
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT_PART_1: &str = r"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

  #[test]
  fn test_part1() {
    assert_eq!(127, parse_part_1(INPUT_PART_1, 5));
  }

  #[test]
  fn test_part2() {
    assert_eq!(62, parse_part_2(INPUT_PART_1, 127))
  }
}
