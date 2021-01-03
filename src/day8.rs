use std::collections::HashSet;
use std::error::Error;

#[derive(Clone)]
enum Action {
  Noop(i64),
  Jump(i64),
  Acc(i64),
}

fn parse_line(input: &str) -> Action {
  let sections: Vec<&str> = input.split(" ").collect();
  let num: i64 = sections[1].parse().unwrap();
  match sections[0] {
    "nop" => Action::Noop(num),
    "acc" => Action::Acc(num),
    "jmp" => Action::Jump(num),
    _ => panic!("Cannot parse input"),
  }
}

fn parse_part_1(input: &str) -> i64 {
  let program: Vec<Action> = input.lines().map(parse_line).collect();
  let mut visited: HashSet<i64> = HashSet::new();
  let mut cursor: i64 = 0;
  let mut acc: i64 = 0;

  loop {
    if visited.contains(&cursor) {
      break;
    }
    visited.insert(cursor);
    let action = &program[cursor as usize];
    match action {
      &Action::Noop(_) => {
        cursor += 1;
        continue;
      }
      &Action::Jump(val) => {
        cursor += val;
        continue;
      }
      &Action::Acc(val) => {
        acc += val;
        cursor += 1;
      }
    }
  }

  acc
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_1(input) as i64)
}

fn does_execute(program: &Vec<Action>) -> (bool, i64) {
  let mut acc: i64 = 0;
  let mut cursor: i64 = 0;
  let mut visited: HashSet<i64> = HashSet::new();

  loop {
    if let None = program.get(cursor as usize) {
      return (true, acc);
    }

    if visited.contains(&cursor) {
      return (false, 0);
    }
    visited.insert(cursor);
    let action = &program[cursor as usize];
    match action {
      &Action::Noop(_) => {
        cursor += 1;
        continue;
      }
      &Action::Jump(val) => {
        cursor += val;
        continue;
      }
      &Action::Acc(val) => {
        acc += val;
        cursor += 1;
      }
    }
  }
}

fn parse_part_2(input: &str) -> i64 {
  let main: Vec<Action> = input.lines().map(parse_line).collect();
  let mut changed: Vec<Action> = main.clone();
  for (i, action) in main.iter().enumerate() {
    match action {
      &Action::Noop(val) => {
        changed[i] = Action::Jump(val);
        let (executes, result) = does_execute(&changed);
        if executes {
          return result;
        }
        changed[i] = Action::Noop(val);
      }
      &Action::Jump(val) => {
        changed[i] = Action::Noop(val);
        let (executes, result) = does_execute(&changed);
        if executes {
          return result;
        }
        changed[i] = Action::Jump(val);
      }
      &Action::Acc(_) => continue,
    }
  }
  panic!("No result found!");
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_2(input) as i64)
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT_PART_1: &str = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

  #[test]
  fn test_part1() {
    assert_eq!(5, parse_part_1(INPUT_PART_1));
  }

  #[test]
  fn test_part2() {
    assert_eq!(8, parse_part_2(INPUT_PART_1));
  }
}
