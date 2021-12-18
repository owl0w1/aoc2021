const CYCLE: usize = 7;
const COOLDOWN: usize = 2;

fn simulate(input: &str, days: usize) -> u64 {
    let mut fish = [0; CYCLE + COOLDOWN];
    input
        .split_terminator(&[',', '\n'][..])
        .map(|s| s.parse::<usize>().unwrap())
        .for_each(|d| fish[d] += 1);
    for day in 0..days {
        fish[(day + CYCLE) % (CYCLE + COOLDOWN)] += fish[day % (CYCLE + COOLDOWN)];
    }
    fish.into_iter().sum()
}

pub fn part1(input: &str) -> u64 {
    simulate(input, 80)
}

pub fn part2(input: &str) -> u64 {
    simulate(input, 256)
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day06.txt").unwrap()),
        352151
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day06.txt").unwrap()),
        1601616884019
    );
}
