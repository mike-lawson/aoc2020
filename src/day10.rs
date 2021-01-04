use std::collections::HashMap;
use std::error::Error;

fn parse_part_1(input: &str) -> i64 {
  let mut numbers: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
  numbers.sort();

  let mut one_difference = 0;
  let mut three_difference = 1;
  for (i, num) in numbers.iter().enumerate() {
    let mut difference = *num;
    if i != 0 {
      difference = num - numbers[i - 1];
    }
    if difference == 1 {
      one_difference += 1;
    } else if difference == 3 {
      three_difference += 1;
    }
  }

  one_difference * three_difference
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_1(input) as i64)
}

pub fn possibilities(sorted: &[i64], cache: &mut HashMap<i64, i64>) -> i64 {
  if let Some(&val) = cache.get(&sorted[0]) {
    return val;
  }
  let len = sorted.len();
  if len == 0 {
    return 0;
  }
  let number = sorted[0];
  let mut total = 0;
  let mut i = 1;
  loop {
    if sorted[i] - number > 3 {
      cache.insert(number, total);
      return total;
    }
    if len == i + 1 {
      cache.insert(number, total + 1);
      return total + 1;
    }
    total += possibilities(&sorted[i..], cache);
    i += 1;
  }
}

pub fn parse_part_2(input: &str) -> i64 {
  let mut numbers: Vec<i64> = input.lines().map(|x| x.parse().unwrap()).collect();
  // Prepare - adding 0 to the beginning, sorting, and adding max + 3 to the end
  numbers.push(0);
  numbers.sort();
  numbers.push(numbers[numbers.len() - 1] + 3);

  let mut cache = HashMap::new();

  possibilities(&numbers, &mut cache)
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_2(input) as i64)
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT_EXAMPLE_1: &str = r"16
10
15
5
1
11
7
19
6
12
4";

  static INPUT_EXAMPLE_2: &str = r"28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

  #[test]
  fn test_part1() {
    assert_eq!(7 * 5, parse_part_1(INPUT_EXAMPLE_1));
    assert_eq!(22 * 10, parse_part_1(INPUT_EXAMPLE_2));
  }

  #[test]
  fn test_part2() {
    assert_eq!(8, parse_part_2(INPUT_EXAMPLE_1));
    assert_eq!(19208, parse_part_2(INPUT_EXAMPLE_2));
  }
}
