pub struct Input {
  pub day: u32,
  pub part: u32,
  pub input: String,
}

mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

pub fn run(input: Input) -> Result<i64, Box<dyn std::error::Error>> {
  let Input { day, part, input } = input;
  match (day, part) {
    (1, 1) => Ok(day1::part1(&input)?),
    (1, 2) => Ok(day1::part2(&input)?),
    (2, 1) => Ok(day2::part1(&input)?),
    (2, 2) => Ok(day2::part2(&input)?),
    (3, 1) => Ok(day3::part1(&input)?),
    (3, 2) => Ok(day3::part2(&input)?),
    (4, 1) => Ok(day4::part1(&input)?),
    (4, 2) => Ok(day4::part2(&input)?),
    (5, 1) => Ok(day5::part1(&input)?),
    (5, 2) => Ok(day5::part2(&input)?),
    (6, 1) => Ok(day6::part1(&input)?),
    (6, 2) => Ok(day6::part2(&input)?),
    (7, 1) => Ok(day7::part1(&input)?),
    (7, 2) => Ok(day7::part2(&input)?),
    (8, 1) => Ok(day8::part1(&input)?),
    (8, 2) => Ok(day8::part2(&input)?),
    (9, 1) => Ok(day9::part1(&input)?),
    (9, 2) => Ok(day9::part2(&input)?),
    (10, 1) => Ok(day10::part1(&input)?),
    (10, 2) => Ok(day10::part2(&input)?),
    (11, 1) => Ok(day11::part1(&input)?),
    (11, 2) => Ok(day11::part2(&input)?),
    (12, 1) => Ok(day12::part1(&input)?),
    _ => Err(format!("{}-{} not found", day, part).into()),
  }
}
