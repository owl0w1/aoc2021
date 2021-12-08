pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub enum Puzzle {
    Day1Part1,
    Day1Part2,
    Day2Part1,
    Day2Part2,
    Day3Part1,
    Day3Part2,
    Day4Part1,
    Day4Part2,
    Day5Part1,
    Day5Part2,
    Day6Part1,
    Day6Part2,
    Day7Part1,
    Day7Part2,
    Day8Part1,
    Day8Part2,
    Day9Part1,
    Day9Part2,
    Day10Part1,
    Day10Part2,
    Day11Part1,
    Day11Part2,
    Day12Part1,
    Day12Part2,
    Day13Part1,
    Day13Part2,
    Day14Part1,
    Day14Part2,
    Day15Part1,
    Day15Part2,
    Day16Part1,
    Day16Part2,
    Day17Part1,
    Day17Part2,
    Day18Part1,
    Day18Part2,
    Day19Part1,
    Day19Part2,
    Day20Part1,
    Day20Part2,
    Day21Part1,
    Day21Part2,
    Day22Part1,
    Day22Part2,
    Day23Part1,
    Day23Part2,
    Day24Part1,
    Day24Part2,
    Day25Part1,
}
use Puzzle::*;

impl Puzzle {
    pub fn solve(&self, input: &str) -> Box<dyn std::fmt::Debug> {
        #[allow(unreachable_code)]
        match self {
            Day1Part1 => Box::new(day01::part1(input)),
            Day1Part2 => Box::new(day01::part2(input)),
            Day2Part1 => Box::new(day02::part1(input)),
            Day2Part2 => Box::new(day02::part2(input)),
            Day3Part1 => Box::new(day03::part1(input)),
            Day3Part2 => Box::new(day03::part2(input)),
            Day4Part1 => Box::new(day04::part1(input)),
            Day4Part2 => Box::new(day04::part2(input)),
            Day5Part1 => Box::new(day05::part1(input)),
            Day5Part2 => Box::new(day05::part2(input)),
            Day6Part1 => Box::new(day06::part1(input)),
            Day6Part2 => Box::new(day06::part2(input)),
            Day7Part1 => Box::new(day07::part1(input)),
            Day7Part2 => Box::new(day07::part2(input)),
            Day8Part1 => Box::new(day08::part1(input)),
            Day8Part2 => Box::new(day08::part2(input)),
            Day9Part1 => Box::new(day09::part1(input)),
            Day9Part2 => Box::new(day09::part2(input)),
            Day10Part1 => Box::new(day10::part1(input)),
            Day10Part2 => Box::new(day10::part2(input)),
            Day11Part1 => Box::new(day11::part1(input)),
            Day11Part2 => Box::new(day11::part2(input)),
            Day12Part1 => Box::new(day12::part1(input)),
            Day12Part2 => Box::new(day12::part2(input)),
            Day13Part1 => Box::new(day13::part1(input)),
            Day13Part2 => Box::new(day13::part2(input)),
            Day14Part1 => Box::new(day14::part1(input)),
            Day14Part2 => Box::new(day14::part2(input)),
            Day15Part1 => Box::new(day15::part1(input)),
            Day15Part2 => Box::new(day15::part2(input)),
            Day16Part1 => Box::new(day16::part1(input)),
            Day16Part2 => Box::new(day16::part2(input)),
            Day17Part1 => Box::new(day17::part1(input)),
            Day17Part2 => Box::new(day17::part2(input)),
            Day18Part1 => Box::new(day18::part1(input)),
            Day18Part2 => Box::new(day18::part2(input)),
            Day19Part1 => Box::new(day19::part1(input)),
            Day19Part2 => Box::new(day19::part2(input)),
            Day20Part1 => Box::new(day20::part1(input)),
            Day20Part2 => Box::new(day20::part2(input)),
            Day21Part1 => Box::new(day21::part1(input)),
            Day21Part2 => Box::new(day21::part2(input)),
            Day22Part1 => Box::new(day22::part1(input)),
            Day22Part2 => Box::new(day22::part2(input)),
            Day23Part1 => Box::new(day23::part1(input)),
            Day23Part2 => Box::new(day23::part2(input)),
            Day24Part1 => Box::new(day24::part1(input)),
            Day24Part2 => Box::new(day24::part2(input)),
            Day25Part1 => Box::new(day25::part1(input)),
        }
    }
}

impl std::convert::TryFrom<(u8, u8)> for Puzzle {
    type Error = &'static str;
    fn try_from(day_part: (u8, u8)) -> Result<Self, Self::Error> {
        match day_part {
            (1, 1) => Ok(Day1Part1),
            (1, 2) => Ok(Day1Part2),
            (2, 1) => Ok(Day2Part1),
            (2, 2) => Ok(Day2Part2),
            (3, 1) => Ok(Day3Part1),
            (3, 2) => Ok(Day3Part2),
            (4, 1) => Ok(Day4Part1),
            (4, 2) => Ok(Day4Part2),
            (5, 1) => Ok(Day5Part1),
            (5, 2) => Ok(Day5Part2),
            (6, 1) => Ok(Day6Part1),
            (6, 2) => Ok(Day6Part2),
            (7, 1) => Ok(Day7Part1),
            (7, 2) => Ok(Day7Part2),
            (8, 1) => Ok(Day8Part1),
            (8, 2) => Ok(Day8Part2),
            (9, 1) => Ok(Day9Part1),
            (9, 2) => Ok(Day9Part2),
            (10, 1) => Ok(Day10Part1),
            (10, 2) => Ok(Day10Part2),
            (11, 1) => Ok(Day11Part1),
            (11, 2) => Ok(Day11Part2),
            (12, 1) => Ok(Day12Part1),
            (12, 2) => Ok(Day12Part2),
            (13, 1) => Ok(Day13Part1),
            (13, 2) => Ok(Day13Part2),
            (14, 1) => Ok(Day14Part1),
            (14, 2) => Ok(Day14Part2),
            (15, 1) => Ok(Day15Part1),
            (15, 2) => Ok(Day15Part2),
            (16, 1) => Ok(Day16Part1),
            (16, 2) => Ok(Day16Part2),
            (17, 1) => Ok(Day17Part1),
            (17, 2) => Ok(Day17Part2),
            (18, 1) => Ok(Day18Part1),
            (18, 2) => Ok(Day18Part2),
            (19, 1) => Ok(Day19Part1),
            (19, 2) => Ok(Day19Part2),
            (20, 1) => Ok(Day20Part1),
            (20, 2) => Ok(Day20Part2),
            (21, 1) => Ok(Day21Part1),
            (21, 2) => Ok(Day21Part2),
            (22, 1) => Ok(Day22Part1),
            (22, 2) => Ok(Day22Part2),
            (23, 1) => Ok(Day23Part1),
            (23, 2) => Ok(Day23Part2),
            (24, 1) => Ok(Day24Part1),
            (24, 2) => Ok(Day24Part2),
            (25, 1) => Ok(Day25Part1),
            (25, 2) => Err("Day 25 does not have Part 2"),
            _ => Err("Day must be in range [1, 25] and Part must be 1 or 2"),
        }
    }
}
