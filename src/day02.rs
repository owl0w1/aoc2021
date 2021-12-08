pub fn part1(input: &str) -> u32 {
    let (mut horizontal_pos, mut depth) = (0, 0);
    for command in input.lines() {
        match command.as_bytes()[0] {
            b'f' => {
                let delta: u32 = command[8..].parse().unwrap();
                horizontal_pos += delta;
            }
            b'u' => {
                let delta: u32 = command[3..].parse().unwrap();
                depth -= delta;
            }
            b'd' => {
                let delta: u32 = command[5..].parse().unwrap();
                depth += delta;
            }
            _ => panic!("invalid input"),
        }
    }
    horizontal_pos * depth
}

pub fn part2(input: &str) -> u32 {
    let (mut horizontal_pos, mut depth) = (0, 0);
    let mut aim = 0;
    for command in input.lines() {
        match command.as_bytes()[0] {
            b'f' => {
                let delta: u32 = command[8..].parse().unwrap();
                horizontal_pos += delta;
                depth += aim * delta;
            }
            b'u' => {
                let delta: u32 = command[3..].parse().unwrap();
                aim -= delta;
            }
            b'd' => {
                let delta: u32 = command[5..].parse().unwrap();
                aim += delta;
            }
            _ => panic!("invalid input"),
        }
    }
    horizontal_pos * depth
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day02.txt").unwrap()),
        1698735
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day02.txt").unwrap()),
        1594785890
    );
}
