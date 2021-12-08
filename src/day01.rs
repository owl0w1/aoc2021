pub fn part1(input: &str) -> u32 {
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .fold((0, u32::MAX), |(count, prev), curr| {
            if prev < curr {
                (count + 1, curr)
            } else {
                (count, curr)
            }
        })
        .0
}

pub fn part2(input: &str) -> u32 {
    input
        .split_ascii_whitespace()
        .filter_map(|s| s.parse().ok())
        .map({
            let mut window = (0, 0, 0);
            move |curr| {
                window = (window.1, window.2, curr);
                window.0 + window.1 + window.2
            }
        })
        .skip(2)
        .fold((0, u32::MAX), |(count, prev), curr| {
            if prev < curr {
                (count + 1, curr)
            } else {
                (count, curr)
            }
        })
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
