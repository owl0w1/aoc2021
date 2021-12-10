pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut score = 0;
        let mut stack = Vec::with_capacity(1 << 5);
        for c in line.as_bytes() {
            match *c {
                b')' => {
                    if stack.pop() != Some(b'(') {
                        score = 3;
                        break;
                    }
                }
                b']' => {
                    if stack.pop() != Some(b'[') {
                        score = 57;
                        break;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        score = 1197;
                        break;
                    }
                }
                b'>' => {
                    if stack.pop() != Some(b'<') {
                        score = 25137;
                        break;
                    }
                }
                _ => stack.push(*c),
            }
        }
        sum += score;
    }
    sum
}

pub fn part2(input: &str) -> u64 {
    let mut scores = Vec::new();
    'lines: for line in input.lines() {
        let mut stack = Vec::with_capacity(1 << 5);
        for c in line.as_bytes() {
            match *c {
                b')' => {
                    if stack.pop() != Some(b'(') {
                        continue 'lines;
                    }
                }
                b']' => {
                    if stack.pop() != Some(b'[') {
                        continue 'lines;
                    }
                }
                b'}' => {
                    if stack.pop() != Some(b'{') {
                        continue 'lines;
                    }
                }
                b'>' => {
                    if stack.pop() != Some(b'<') {
                        continue 'lines;
                    }
                }
                _ => stack.push(*c),
            }
        }
        let mut score = 0;
        for c in stack.into_iter().rev() {
            score *= 5;
            score += match c {
                b'(' => 1,
                b'[' => 2,
                b'{' => 3,
                b'<' => 4,
                _ => panic!("invalid input"),
            };
        }
        scores.push(score);
    }
    let mid = scores.len() / 2;
    *scores.select_nth_unstable(mid).1
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day10.txt").unwrap()),
        319233
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day10.txt").unwrap()),
        1118976874
    );
}
