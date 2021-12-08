const TITLE: &str = "Advent of Code 20xx solutions";
const USAGE_INFO: &str = "\
Usage:
    aoc20xx <day> <part>

Arguments:
    day  {1 .. =25}
    part {1 | 2}";

use aoc20xx::*;
use std::{
    env::args,
    io::{stdin, Read},
    process::exit,
};

fn main() {
    let puzzle = parse_arguments();
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();
    println!("{:?}", puzzle.solve(&input));
}

fn parse_arguments() -> Puzzle {
    let args: Vec<_> = args().skip(1).collect();
    match args.len() {
        0 => {
            eprintln!("{}\n", TITLE);
            eprintln!("{}", USAGE_INFO);
            exit(1);
        }
        1 => {
            eprintln!("Missing argument");
            exit(1);
        }
        2 => (),
        _ => {
            eprintln!("Too many arguments");
            exit(1);
        }
    }
    let day = match args[0].parse() {
        Ok(day @ 1..=25) => day,
        _ => {
            eprintln!("Day must be an integer in range [1, 25]");
            exit(1);
        }
    };
    let part = match args[1].parse() {
        Ok(part @ 1..=2) => part,
        _ => {
            eprintln!("Part must be 1 or 2");
            exit(1);
        }
    };
    if (day, part) == (25, 2) {
        eprintln!("Day 25 does not have Part 2");
        exit(1);
    }
    (day, part).try_into().unwrap()
}
