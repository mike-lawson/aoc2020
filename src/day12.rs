use std::error::Error;
use std::fs;

#[derive(Debug, PartialEq)]
enum Action {
  North(i64),
  East(i64),
  South(i64),
  West(i64),
  Left(i64),
  Right(i64),
  Forward(i64),
}

// (north, east, facing)
type Tuple = (i64, i64, i64);

fn parse_action(input: &str) -> Action {
  let (action, value) = input.split_at(1);
  let value: i64 = value.parse().unwrap();
  match action {
    "N" => Action::North(value),
    "E" => Action::East(value),
    "S" => Action::South(value),
    "W" => Action::West(value),
    "L" => Action::Left(value),
    "R" => Action::Right(value),
    "F" => Action::Forward(value),
    _ => panic!("Unexpected input"),
  }
}

fn process_direction(tuple: &mut Tuple, action: Action) {
  match action {
    Action::North(val) => {
      tuple.0 += val;
    }
    Action::South(val) => {
      tuple.0 -= val;
    }
    Action::East(val) => {
      tuple.1 += val;
    }
    Action::West(val) => {
      tuple.1 -= val;
    }
    Action::Left(val) => {
      tuple.2 = tuple.2 - val;
      if tuple.2 < 0 {
        tuple.2 += 360;
      }
    }
    Action::Right(val) => {
      tuple.2 = (tuple.2 + val) % 360;
    }
    Action::Forward(val) => match tuple.2 {
      0 => process_direction(tuple, Action::North(val)),
      90 => process_direction(tuple, Action::East(val)),
      180 => process_direction(tuple, Action::South(val)),
      270 => process_direction(tuple, Action::West(val)),
      _ => panic!(format!("Unexpected direction {}", tuple.2)),
    },
  }
}

fn process_correct_directions(ship: &mut Tuple, waypoint: &mut Tuple, action: Action) {
  match action {
    Action::North(val) => {
      waypoint.0 += val;
    }
    Action::South(val) => {
      waypoint.0 -= val;
    }
    Action::East(val) => {
      waypoint.1 += val;
    }
    Action::West(val) => {
      waypoint.1 -= val;
    }
    Action::Left(val) => match val {
      90 => {
        let temp = waypoint.0;
        waypoint.0 = waypoint.1;
        waypoint.1 = -temp;
      }
      180 => {
        waypoint.0 = -waypoint.0;
        waypoint.1 = -waypoint.1;
      }
      270 => {
        let temp = waypoint.0;
        waypoint.0 = -waypoint.1;
        waypoint.1 = temp;
      }
      _ => todo!(),
    },
    Action::Right(val) => process_correct_directions(ship, waypoint, Action::Left(360 - val)),
    Action::Forward(val) => {
      ship.0 += val * waypoint.0;
      ship.1 += val * waypoint.1;
    }
  }
}

fn parse_part_1(input: &str) -> i64 {
  let actions: Vec<Action> = input.lines().map(|x| parse_action(x)).collect();
  let mut tuple: Tuple = (0, 0, 90);
  for action in actions {
    process_direction(&mut tuple, action);
  }

  tuple.0.abs() + tuple.1.abs()
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_1(input))
}

pub fn parse_part_2(input: &str) -> i64 {
  let actions: Vec<Action> = input.lines().map(|x| parse_action(x)).collect();
  let mut ship: Tuple = (0, 0, 90);
  let mut waypoint: Tuple = (1, 10, 0);
  for action in actions {
    process_correct_directions(&mut ship, &mut waypoint, action);
  }

  ship.0.abs() + ship.1.abs()
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_2(input))
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT_EXAMPLE_1: &str = r"F10
N3
F7
R90
F11";

  #[test]
  fn test_parse_action() {
    assert_eq!(Action::East(50), parse_action("E50"));
  }

  #[test]
  fn test_part1() {
    assert_eq!(25, parse_part_1(INPUT_EXAMPLE_1));
  }

  #[test]
  fn test_part2() {
    let input = fs::read_to_string("input/day12-test.txt").unwrap();
    assert_eq!(286, parse_part_2(INPUT_EXAMPLE_1));
    assert_eq!(30761, parse_part_2(&input));
  }
}
