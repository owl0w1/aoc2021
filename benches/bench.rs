use aoc20xx::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::read_to_string;

fn bench(c: &mut Criterion) {
    if let Ok(input) = read_to_string("input/day01.txt") {
        c.bench_function("day01::part1", |b| {
            b.iter(|| day01::part1(black_box(&input)))
        });
        c.bench_function("day01::part2", |b| {
            b.iter(|| day01::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day02.txt") {
        c.bench_function("day02::part1", |b| {
            b.iter(|| day02::part1(black_box(&input)))
        });
        c.bench_function("day02::part2", |b| {
            b.iter(|| day02::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day03.txt") {
        c.bench_function("day03::part1", |b| {
            b.iter(|| day03::part1(black_box(&input)))
        });
        c.bench_function("day03::part2", |b| {
            b.iter(|| day03::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day04.txt") {
        c.bench_function("day04::part1", |b| {
            b.iter(|| day04::part1(black_box(&input)))
        });
        c.bench_function("day04::part2", |b| {
            b.iter(|| day04::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day05.txt") {
        c.bench_function("day05::part1", |b| {
            b.iter(|| day05::part1(black_box(&input)))
        });
        c.bench_function("day05::part2", |b| {
            b.iter(|| day05::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day06.txt") {
        c.bench_function("day06::part1", |b| {
            b.iter(|| day06::part1(black_box(&input)))
        });
        c.bench_function("day06::part2", |b| {
            b.iter(|| day06::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day07.txt") {
        c.bench_function("day07::part1", |b| {
            b.iter(|| day07::part1(black_box(&input)))
        });
        c.bench_function("day07::part2", |b| {
            b.iter(|| day07::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day08.txt") {
        c.bench_function("day08::part1", |b| {
            b.iter(|| day08::part1(black_box(&input)))
        });
        c.bench_function("day08::part2", |b| {
            b.iter(|| day08::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day09.txt") {
        c.bench_function("day09::part1", |b| {
            b.iter(|| day09::part1(black_box(&input)))
        });
        c.bench_function("day09::part2", |b| {
            b.iter(|| day09::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day10.txt") {
        c.bench_function("day10::part1", |b| {
            b.iter(|| day10::part1(black_box(&input)))
        });
        c.bench_function("day10::part2", |b| {
            b.iter(|| day10::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day11.txt") {
        c.bench_function("day11::part1", |b| {
            b.iter(|| day11::part1(black_box(&input)))
        });
        c.bench_function("day11::part2", |b| {
            b.iter(|| day11::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day12.txt") {
        c.bench_function("day12::part1", |b| {
            b.iter(|| day12::part1(black_box(&input)))
        });
        c.bench_function("day12::part2", |b| {
            b.iter(|| day12::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day13.txt") {
        c.bench_function("day13::part1", |b| {
            b.iter(|| day13::part1(black_box(&input)))
        });
        c.bench_function("day13::part2", |b| {
            b.iter(|| day13::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day14.txt") {
        c.bench_function("day14::part1", |b| {
            b.iter(|| day14::part1(black_box(&input)))
        });
        c.bench_function("day14::part2", |b| {
            b.iter(|| day14::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day15.txt") {
        c.bench_function("day15::part1", |b| {
            b.iter(|| day15::part1(black_box(&input)))
        });
        c.bench_function("day15::part2", |b| {
            b.iter(|| day15::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day16.txt") {
        c.bench_function("day16::part1", |b| {
            b.iter(|| day16::part1(black_box(&input)))
        });
        c.bench_function("day16::part2", |b| {
            b.iter(|| day16::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day17.txt") {
        c.bench_function("day17::part1", |b| {
            b.iter(|| day17::part1(black_box(&input)))
        });
        c.bench_function("day17::part2", |b| {
            b.iter(|| day17::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day18.txt") {
        c.bench_function("day18::part1", |b| {
            b.iter(|| day18::part1(black_box(&input)))
        });
        c.bench_function("day18::part2", |b| {
            b.iter(|| day18::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day19.txt") {
        c.bench_function("day19::part1", |b| {
            b.iter(|| day19::part1(black_box(&input)))
        });
        c.bench_function("day19::part2", |b| {
            b.iter(|| day19::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day20.txt") {
        c.bench_function("day20::part1", |b| {
            b.iter(|| day20::part1(black_box(&input)))
        });
        c.bench_function("day20::part2", |b| {
            b.iter(|| day20::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day21.txt") {
        c.bench_function("day21::part1", |b| {
            b.iter(|| day21::part1(black_box(&input)))
        });
        c.bench_function("day21::part2", |b| {
            b.iter(|| day21::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day22.txt") {
        c.bench_function("day22::part1", |b| {
            b.iter(|| day22::part1(black_box(&input)))
        });
        c.bench_function("day22::part2", |b| {
            b.iter(|| day22::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day23.txt") {
        c.bench_function("day23::part1", |b| {
            b.iter(|| day23::part1(black_box(&input)))
        });
        c.bench_function("day23::part2", |b| {
            b.iter(|| day23::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day24.txt") {
        c.bench_function("day24::part1", |b| {
            b.iter(|| day24::part1(black_box(&input)))
        });
        c.bench_function("day24::part2", |b| {
            b.iter(|| day24::part2(black_box(&input)))
        });
    }
    if let Ok(input) = read_to_string("input/day25.txt") {
        c.bench_function("day25::part1", |b| {
            b.iter(|| day25::part1(black_box(&input)))
        });
    }
}

criterion_group!(benches, bench);
criterion_main!(benches);
