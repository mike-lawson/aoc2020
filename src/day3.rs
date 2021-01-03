use std::error::Error;

// x, y (right, down)
struct Point(usize, usize);

struct Map {
  trees: Vec<Point>,
  columns: usize,
}

impl Map {
  fn count_trees(&self, slope: &Point) -> usize {
    self
      .trees
      .iter()
      .filter(|point| {
        point.1 * slope.0 % (self.columns * slope.1) == point.0 * slope.1 % (self.columns * slope.1)
      })
      .count()
  }
}

fn parse_input(input: &str) -> Map {
  let mut trees = Vec::new();
  let mut columns = 0;

  for (down, line) in input.lines().enumerate() {
    columns = 0;
    for (right, c) in line.chars().enumerate() {
      if c == '#' {
        trees.push(Point(right, down));
      }
      columns += 1;
    }
  }
  Map { columns, trees }
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  let map = parse_input(input);
  Ok(map.count_trees(&Point(3, 1)) as i64)
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  let map = parse_input(input);
  let points = vec![
    Point(1, 1),
    Point(3, 1),
    Point(5, 1),
    Point(7, 1),
    Point(1, 2),
  ];

  Ok(
    points
      .iter()
      .fold(1, |acc, point| acc * map.count_trees(point)) as i64,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT: &str = r"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

  #[test]
  fn tutorial_passes() {
    assert_eq!(7, part1(INPUT).unwrap());
  }

  #[test]
  fn part2_passes() {
    assert_eq!(336, part2(INPUT).unwrap());
  }
}
