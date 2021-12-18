pub fn part1(input: &str) -> u32 {
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .fold((0, u32::MAX), |(count, prev), curr| {
            (count + (prev < curr) as u32, curr)
        })
        .0
}

pub fn part2(input: &str) -> u32 {
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .fold(
            (0, (u32::MAX, u32::MAX, u32::MAX)),
            |(count, prev), curr| (count + (prev.0 < curr) as u32, (prev.1, prev.2, curr)),
        )
        .0
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day01.txt").unwrap()),
        1466
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day01.txt").unwrap()),
        1491
    );
}
