use std::cell::RefCell;
use std::collections::hash_set::HashSet;
use std::error::Error;

struct Answers {
  letter: char,
  count: usize,
}

fn parse_part_1(input: &str) -> usize {
  let groups: Vec<&str> = input.split("\n\n").collect();
  let mut total = 0;
  for group in groups {
    let mut set = HashSet::new();
    for c in group.chars() {
      if c == '\n' {
        continue;
      }
      set.insert(c);
    }
    total += set.into_iter().count();
  }
  total
}

fn parse_part_2(input: &str) -> usize {
  let groups: Vec<&str> = input.split("\n\n").collect();
  let mut total = 0;

  for group in groups {
    let mut vec: Vec<RefCell<Answers>> = Vec::new();
    let persons = group.split("\n").count();

    for c in group.chars() {
      if c == '\n' {
        continue;
      }
      if let Some(answer) = vec.iter().find(|x| x.borrow().letter == c) {
        answer.borrow_mut().count += 1;
      } else {
        vec.push(RefCell::new(Answers {
          letter: c,
          count: 1,
        }));
      }
    }
    total += vec.iter().filter(|x| x.borrow().count == persons).count();
  }

  total
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_1(input) as i64)
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_2(input) as i64)
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT: &str = r"abc

a
b
c

ab
ac

a
a
a
a

b";

  #[test]
  fn test_part_1() {
    assert_eq!(11, parse_part_1(INPUT));
  }

  #[test]
  fn test_part_2() {
    assert_eq!(6, parse_part_2(INPUT));
  }
}
