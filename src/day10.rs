use std::error::Error;

fn parse_part_1(input: &str) -> i64 {
  3
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_1(input) as i64)
}

// pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
//   Ok(parse_part_2(input, part1(input).unwrap()) as i64)
// }

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
    assert_eq!(127, parse_part_1(INPUT_PART_1));
  }

  // #[test]
  // fn test_part2() {
  //   assert_eq!(62, parse_part_2(INPUT_PART_1, 127))
  // }
}
