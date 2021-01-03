use std::error::Error;

#[derive(Debug, PartialEq)]
struct Pass {
  row: i64,
  column: i64,
  id: i64,
}

fn construct_possibles() -> Vec<Pass> {
  let mut vec = Vec::new();
  for row in 0..127 {
    for column in 0..7 {
      vec.push(Pass {
        row,
        column,
        id: row * 8 + column,
      })
    }
  }

  vec
}

fn parse_input(input: &str) -> Vec<Pass> {
  input.lines().map(|x| parse_seat(x)).collect()
}

fn parse_seat(input: &str) -> Pass {
  let row_code = &input[..7];
  let column_code = &input[7..];
  let mut row = 0;
  for (i, fb) in row_code.chars().rev().enumerate() {
    if fb == 'B' {
      row += i64::pow(2, i as u32);
    }
  }

  let mut column = 0;
  for (i, rl) in column_code.chars().rev().enumerate() {
    if rl == 'R' {
      column += i64::pow(2, i as u32);
    }
  }

  Pass {
    row,
    column,
    id: row * 8 + column,
  }
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(
    parse_input(input)
      .iter()
      .fold(0, |acc, pass| i64::max(acc, pass.id)),
  )
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  let possibles = construct_possibles();
  let actuals = parse_input(input);

  let mut potentials = Vec::new();
  for possible in possibles {
    if !actuals.contains(&possible) {
      potentials.push(possible);
    }
  }

  let mut found = 0;

  for potential in potentials {
    if actuals.iter().any(|x| x.id == potential.id + 1)
      && actuals.iter().any(|x| x.id == potential.id - 1)
    {
      found = potential.id;
    }
  }

  Ok(found)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_parse_seat() {
    let tests = vec![
      (
        "FBFBBFFRLR",
        Pass {
          row: 44,
          column: 5,
          id: 357,
        },
      ),
      (
        "BFFFBBFRRR",
        Pass {
          row: 70,
          column: 7,
          id: 567,
        },
      ),
      (
        "FFFBBBFRRR",
        Pass {
          row: 14,
          column: 7,
          id: 119,
        },
      ),
      (
        "BBFFBBFRLL",
        Pass {
          row: 102,
          column: 4,
          id: 820,
        },
      ),
    ];

    for (code, pass) in tests {
      assert_eq!(parse_seat(code), pass);
    }
  }
}
