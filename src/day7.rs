use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Bag {
  prefix: String,
  color: String,
}

impl Bag {
  fn hash(&self) -> String {
    format!("{}{}", self.prefix, self.color)
  }
}

lazy_static! {
  static ref LEFT_REGEX: Regex = Regex::new(r"^(\w+)\s(\w+)\sbags?\s?$").unwrap();
  static ref RIGHT_REGEX: Regex =
    Regex::new(r"^(?:contain\s)?(?:(?P<count>\d)\s(?P<prefix>\w+)\s(?P<color>\w+)\s\w+[,.]?\s?)+$")
      .unwrap();
}

// Parses into an adjacency list - used exclusively in part 2
fn parse_input_list(input: &str) -> Vec<(Bag, Vec<(Bag, usize)>)> {
  let lines: Vec<&str> = input.lines().collect();
  let mut all_edges = Vec::new();

  for line in lines {
    let split_index = line.find("contain").unwrap();
    let (left, right) = line.split_at(split_index);

    // Begin parsing left
    let bag = parse_left(left);
    let edges = parse_right(right);
    all_edges.push((bag, edges));
  }

  all_edges
}

fn parse_left(input: &str) -> Bag {
  let captures = LEFT_REGEX.captures(input).unwrap();
  let prefix = captures
    .get(1)
    .or_else(|| panic!("Could not find prefix"))
    .map(|x| String::from(x.as_str()))
    .unwrap();
  let color = captures
    .get(2)
    .or_else(|| panic!("Could not find color"))
    .map(|x| String::from(x.as_str()))
    .unwrap();

  Bag { prefix, color }
}

fn parse_right(input: &str) -> Vec<(Bag, usize)> {
  let mut result = Vec::new();

  for line in input.split(", ") {
    let captures = RIGHT_REGEX.captures_iter(line);
    for capture in captures {
      let count: usize = capture
        .name("count")
        .map(|x| x.as_str().parse().unwrap())
        .unwrap();
      let prefix = capture
        .name("prefix")
        .map(|x| String::from(x.as_str()))
        .unwrap();
      let color = capture
        .name("color")
        .map(|x| String::from(x.as_str()))
        .unwrap();
      result.push((Bag { prefix, color }, count))
    }
  }

  result
}

// Parses an adjacency list into a matrix - used for part 1
pub fn list_to_matrix<'a>(
  adj_list: Vec<(Bag, Vec<(Bag, usize)>)>,
) -> (HashMap<String, usize>, Vec<Vec<bool>>) {
  let len = adj_list.len();
  let mut matrix = vec![vec![false; len]; len];
  let mut map: HashMap<String, usize> = HashMap::new();

  // First iteration - map bag to usize for lookup purposes
  for (i, (bag, _)) in adj_list.iter().enumerate() {
    map.insert(bag.hash(), i);
  }

  // Second iteration - create matrix
  for (bag, edges) in adj_list {
    let &bag_index = map.get(&bag.hash()).unwrap();
    for (edge_bag, _) in edges {
      let &edge_index = map.get(&edge_bag.hash()).unwrap();
      matrix[bag_index][edge_index] = true;
    }
  }
  (map, matrix)
}

pub fn total(key: String, map: HashMap<String, usize>, matrix: Vec<Vec<bool>>) -> usize {
  let &index = map.get(&key).unwrap();
  let mut set: HashSet<usize> = HashSet::new();
  let mut queue: Vec<usize> = Vec::new();

  let mut num_found = 0;
  // First pass - get first level connections
  for (row_index, row) in matrix.iter().enumerate() {
    if row[index] {
      set.insert(row_index);
      queue.push(row_index);
      num_found += 1;
    }
  }

  // Exhaust queue
  while queue.len() > 0 {
    let index = queue.pop().unwrap();
    for (row_index, row) in matrix.iter().enumerate() {
      if row[index] {
        if set.contains(&row_index) {
          continue;
        }
        set.insert(row_index);
        queue.push(row_index);
        num_found += 1;
      }
    }
  }

  num_found
}

fn parse_part_1(input: &str) -> usize {
  let adj_list = parse_input_list(input);
  let (map, matrix) = list_to_matrix(adj_list);
  total(String::from("shinygold"), map, matrix)
}

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_1(input) as i64)
}

// Recursively returns the amount of bags that the given bag can include (including itself)
fn calculate(
  // cache required for storage of previously executed calculations
  cache: &mut HashMap<String, usize>,
  list: &Vec<(Bag, Vec<(Bag, usize)>)>,
  key: String,
) -> usize {
  if let Some(&val) = cache.get(&key) {
    return val;
  }

  // One for the current bag
  let mut total = 1;
  let (_, edges) = list.into_iter().find(|(bag, _)| bag.hash() == key).unwrap();

  for (bag, count) in edges {
    total += count * calculate(cache, list, bag.hash());
  }

  cache.insert(key, total);
  total
}

fn parse_part_2(input: &str) -> usize {
  let mut cache: HashMap<String, usize> = HashMap::new();
  let adj_list = parse_input_list(input);

  // Subtracting one because we're not counting the bag we're using
  calculate(&mut cache, &adj_list, String::from("shinygold")) - 1
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
  Ok(parse_part_2(input) as i64)
}

// Useful for debugging purposes
#[allow(dead_code)]
pub fn print_data(map: HashMap<String, usize>, matrix: Vec<Vec<bool>>) {
  let len = matrix.len();
  // Print out bag indexs
  for (key, val) in map.iter() {
    println!("{}: {}", key, val);
  }

  // Print out matrix
  print!("  ");
  for i in 0..len {
    print!("{} ", i);
  }
  print!("\n");
  for (x, row) in matrix.iter().enumerate() {
    print!("{} ", x);
    for &val in row {
      if val {
        print!("1 ");
      } else {
        print!("0 ");
      }
    }
    print!("\n");
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  static INPUT_PART_1: &str = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

  #[test]
  fn test_part1() {
    assert_eq!(4, parse_part_1(INPUT_PART_1));
  }

  static INPUT_PART_2: &str = r"shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

  #[test]
  fn test_part2() {
    assert_eq!(32, parse_part_2(INPUT_PART_1));
    assert_eq!(126, parse_part_2(INPUT_PART_2));
  }
}
