#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> ([[u64; 26]; 26], usize, Vec<(usize, usize, usize)>) {
    let polymer_len = input.find('\n').unwrap();
    let mut polymer = [[0; 26]; 26];
    let mut prev_element = (input.as_bytes()[0] - b'A') as usize;
    for letter in &input.as_bytes()[1..polymer_len] {
        let element = (letter - b'A') as usize;
        polymer[prev_element][element] += 1;
        prev_element = element;
    }
    let mut rules = Vec::with_capacity(1 << 8);
    let rules_lines = input.as_bytes()[polymer_len + 2..input.len() - 1].split(|c| *c == b'\n');
    for rule in rules_lines {
        let left = (rule[0] - b'A') as usize;
        let right = (rule[1] - b'A') as usize;
        let insert = (rule[6] - b'A') as usize;
        rules.push((left, right, insert));
    }
    (polymer, prev_element, rules)
}

fn grow(input: &str, steps: u32) -> u64 {
    let (mut polymer, last_element, rules) = parse_input(input);
    for _ in 0..steps {
        let mut next_polymer = polymer;
        for &(left, right, insert) in &rules {
            next_polymer[left][insert] += polymer[left][right];
            next_polymer[insert][right] += polymer[left][right];
            next_polymer[left][right] -= polymer[left][right];
        }
        polymer = next_polymer;
    }
    let mut count = [0; 26];
    for element in 0..26 {
        count[element] = polymer[element].iter().sum();
    }
    count[last_element] += 1;
    count.sort_unstable();
    count[25] - count.into_iter().find(|n| *n != 0).unwrap()
}

pub fn part1(input: &str) -> u64 {
    grow(input, 10)
}

pub fn part2(input: &str) -> u64 {
    grow(input, 40)
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day14.txt").unwrap()),
        2768
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day14.txt").unwrap()),
        2914365137499
    );
}
