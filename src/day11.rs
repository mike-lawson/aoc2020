use std::collections::HashMap;
use std::error::Error;
use std::fmt;

static DIRECTIONS: [(i64, i64); 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

// idx = col + row * rows
#[derive(Copy, Clone, PartialEq, Eq)]
enum Position {
  Floor,
  Occupied,
  Empty,
}

struct Grid {
  map: HashMap<usize, Position>,
  rows: usize,
  cols: usize,
}

impl Grid {
  fn new(input: &str) -> Grid {
    let lines: Vec<&str> = input.lines().collect();
    let rows = lines.len();
    let cols = lines[0].len();
    let mut map = HashMap::new();
    for (row, line) in lines.iter().enumerate() {
      for (col, c) in line.chars().enumerate() {
        let position = match c {
          '.' => Position::Floor,
          'L' => Position::Empty,
          '#' => Position::Occupied,
          _ => panic!("Invalid position given"),
        };
        map.insert(col + row * rows, position);
      }
    }
    Grid { map, rows, cols }
  }

  fn get(&self, row: usize, col: usize) -> Option<&Position> {
    self.map.get(&(col + row * self.rows))
  }

  fn count_occupied(&self) -> u64 {
    let mut total = 0;
    for row in 0..self.rows {
      for col in 0..self.cols {
        if let Some(Position::Occupied) = self.get(row, col) {
          total += 1;
        }
      }
    }

    total
  }

  fn is_occupied(&self, row: usize, col: usize) -> bool {
    if let Some(Position::Occupied) = self.get(row, col) {
      true
    } else {
      false
    }
  }

  fn has_far_occupied_adjacent(&self, row: usize, col: usize, row_mod: i64, col_mod: i64) -> bool {
    let mut r: i64 = row as i64 + row_mod;
    let mut c: i64 = col as i64 + col_mod;
    while r >= 0 && r < self.rows as i64 && c >= 0 && c < self.cols as i64 {
      match self.get(r as usize, c as usize).unwrap() {
        Position::Empty => return false,
        Position::Occupied => return true,
        Position::Floor => {
          r += row_mod;
          c += col_mod;
          continue;
        }
      }
    }
    false
  }

  fn count_occupied_adjacents(&self, row: usize, col: usize) -> u64 {
    let mut total = 0;
    for (r, c) in DIRECTIONS.iter() {
      let test_row = r + row as i64;
      let test_col = c + col as i64;
      if test_col < 0
        || test_row < 0
        || test_row >= self.rows as i64
        || test_col >= self.rows as i64
      {
        continue;
      }
      if self.is_occupied(test_row as usize, test_col as usize) {
        total += 1;
      }
    }

    total
  }

  fn count_occupied_far_adjacents(&self, row: usize, col: usize) -> i64 {
    let mut total = 0;

    for &(r, c) in DIRECTIONS.iter() {
      if self.has_far_occupied_adjacent(row, col, r, c) {
        total += 1;
      }
    }

    total
  }

  fn mutate(&self) -> Grid {
    let mut map: HashMap<usize, Position> = HashMap::new();
    let cols = self.cols;
    let rows = self.rows;
    for row in 0..self.rows {
      for col in 0..self.cols {
        let mut position = *self.get(row, col).unwrap();
        if position != Position::Floor {
          let occupied_adjacents = self.count_occupied_adjacents(row, col);
          if occupied_adjacents == 0 {
            position = Position::Occupied;
          } else if occupied_adjacents >= 4 {
            position = Position::Empty;
          }
        }
        map.insert(col + row * rows, position);
      }
    }
    Grid { map, rows, cols }
  }

  fn mutate_far(&self) -> Grid {
    let mut map: HashMap<usize, Position> = HashMap::new();
    let cols = self.cols;
    let rows = self.rows;
    for row in 0..self.rows {
      for col in 0..self.cols {
        let mut position = *self.get(row, col).unwrap();
        if position != Position::Floor {
          let occupied_adjacents = self.count_occupied_far_adjacents(row, col);
          if occupied_adjacents == 0 {
            position = Position::Occupied;
          } else if occupied_adjacents >= 5 {
            position = Position::Empty;
          }
        }
        map.insert(col + row * rows, position);
      }
    }
    Grid { map, rows, cols }
  }
}

impl PartialEq for Grid {
  fn eq(&self, other: &Self) -> bool {
    if self.cols != other.cols || self.rows != other.rows {
      return false;
    }
    for row in 0..self.rows {
      for col in 0..self.cols {
        if self.get(row, col) != other.get(row, col) {
          return false;
        }
      }
    }
    true
  }
}

impl fmt::Display for Grid {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut grid = String::with_capacity(self.rows * (self.cols + 1));
    for row in 0..self.rows {
      for col in 0..self.cols {
        let marker = match self.get(row, col).unwrap() {
          Position::Floor => '.',
          Position::Occupied => '#',
          Position::Empty => 'L',
        };
        grid.push(marker);
      }
      grid.push('\n');
    }
    write!(f, "{}", grid)
  }
}

fn parse_part_1(input: &str) -> i64 {
  let mut grid = Grid::new(input);
  loop {
    let new_grid = grid.mutate();
    if new_grid == grid {
      return grid.count_occupied() as i64;
    }
    grid = new_grid;
  }
}

fn parse_part_2(input: &str) -> i64 {
  let mut grid = Grid::new(input);
  loop {
    let new_grid = grid.mutate_far();
    if new_grid == grid {
      return grid.count_occupied() as i64;
    }
    grid = new_grid;
  }
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_1(input))
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_2(input))
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT_EXAMPLE_1: &str = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

  static SIMPLE_EXAMPLE: &str = r".#
#.";

  #[test]
  fn test_adjacent() {
    let grid = Grid::new(SIMPLE_EXAMPLE);
    assert_eq!(2, grid.count_occupied_adjacents(0, 0));
    assert_eq!(1, grid.count_occupied_adjacents(0, 1));
  }

  #[test]
  fn test_part1() {
    assert_eq!(37, parse_part_1(INPUT_EXAMPLE_1));
  }
  #[test]
  fn test_part2() {
    assert_eq!(26, parse_part_2(INPUT_EXAMPLE_1));
  }
}
