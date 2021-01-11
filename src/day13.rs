use std::error::Error;

fn parse_input(input: &str) -> (i64, Vec<i64>) {
  let lines: Vec<&str> = input.lines().collect();
  let target: i64 = lines[0].parse().unwrap();
  let buses: Vec<i64> = lines[1]
    .split(',')
    .filter(|x| x != &"x")
    .map(|x| x.parse().unwrap())
    .collect();

  return (target, buses);
}

fn parse_part_1(input: &str) -> i64 {
  let (target, buses) = parse_input(input);

  let mut best: Option<i64> = None;
  let mut answer = 0;
  for bus in buses {
    let diff = (bus * (target / bus + 1)) - target;
    if best == None || diff < best.unwrap() {
      best = Some(diff);
      answer = bus * diff;
    }
  }
  answer
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_1(input))
}

fn parse_offset_buses(input: &str) -> Vec<(i64, i64)> {
  let line: Vec<&str> = input.lines().nth(1).unwrap().split(',').collect();
  let mut buses: Vec<(i64, i64)> = Vec::new();
  for (i, &bus) in line.iter().enumerate() {
    if bus == "x" {
      continue;
    }
    buses.push((i as i64, bus.parse().unwrap()));
  }
  buses
}

// Not used in the solution - takes far, far too long
#[allow(dead_code)]
fn part_2_brute_force(input: &str) -> i64 {
  let buses = parse_offset_buses(input);
  for i in 0..i64::MAX {
    let mut found = true;
    for (x, bus) in buses.iter() {
      if (i + x) % bus != 0 {
        found = false;
        break;
      }
    }
    if found {
      return i;
    }
  }
  panic!("No results found");
}

fn part_2_reduce_search_space(input: &str) -> i64 {
  let buses = parse_offset_buses(input);
  let mut current = buses[0].1;
  let mut next_index = 1;
  loop {
    for (offset, bus) in buses[next_index..].iter() {
      if (current + offset) % bus != 0 {
        break;
      }
      // Increment our next_index "pointer" to the next_index
      next_index += 1;
      if next_index == buses.len() {
        // We found a match!
        return current;
      }
    }
    // Given current equals a number that has been previously found to be divisible by buses[0..next_index]
    // it follows that the next number to check ought to be the current number plus the
    // product of buses[0..next_index] (provided that they are all primes, which they are)
    current += buses[0..next_index].iter().fold(1, |a, (_, bus)| a * bus);
  }
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(part_2_reduce_search_space(input))
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT_EXAMPLE_1: &str = r"939
7,13,x,x,59,x,31,19";

  static PART_TWO_EXAMPLES: &'static [(i64, &str)] = &[
    (3417, "\n17,x,13,19"),
    (754018, "\n67,7,59,61"),
    (779210, "\n67,x,7,59,61"),
    (1261476, "\n67,7,x,59,61"),
    (1202161486, "\n1789,37,47,1889"),
  ];

  #[test]
  fn test_part1() {
    assert_eq!(295, parse_part_1(INPUT_EXAMPLE_1));
  }

  #[test]
  fn test_part2() {
    assert_eq!(1068781, part_2_reduce_search_space(INPUT_EXAMPLE_1));
    for &(expected, input) in PART_TWO_EXAMPLES {
      assert_eq!(expected, part_2_reduce_search_space(input));
    }
  }
}
