use aoc2020;
use std::fs;
use std::process;

fn main() {
    let mut args = std::env::args();
    args.next();

    let day: u32 = args
        .next()
        .expect("Must supply the puzzle day")
        .parse()
        .expect("Cannot parse a number from the day provided");
    let part: u32 = args
        .next()
        .expect("Must supply the puzzle part")
        .parse()
        .expect("Cannot parse a number from the part provided");

    let input = fs::read_to_string(format!("input/day{}.txt", day)).unwrap_or_else(|e| {
        eprintln!("unable to read file: {}", e);
        process::exit(1);
    });

    let output = aoc2020::run(aoc2020::Input { day, part, input });
    match output {
        Ok(val) => println!("{}", val),
        Err(err) => println!("Got err: {}", err),
    }
}
