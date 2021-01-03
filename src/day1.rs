use std::collections::BTreeSet;
use std::error::Error;

pub fn part1(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut set = BTreeSet::new();
    for line in input.lines() {
        let val = str::parse::<i64>(line)?;
        set.insert(val);
        if set.contains(&(2020 - val)) {
            return Ok(val * (2020 - val));
        }
    }
    return Err("Match not found")?;
}

pub fn part2(input: &str) -> Result<i64, Box<dyn Error>> {
    let mut set = BTreeSet::new();
    for line in input.lines() {
        let val = str::parse::<i64>(line)?;
        set.insert(val);
    }
    // This will be sorted
    for low in set.range(0..) {
        for next in set.range(low..) {
            let set_match = 2020 - low - next;
            if set_match < 0 {
                break;
            }
            if set.contains(&set_match) {
                return Ok((low * next * set_match) as i64);
            }
        }
    }
    return Err("Match not found")?;
}
