fn parse_snailfish(s: impl AsRef<[u8]>) -> Vec<(u8, u8)> {
    let mut snailfish = Vec::with_capacity(1 << 4);
    let mut depth = 0;
    for c in s.as_ref() {
        match c {
            b'[' => depth += 1,
            b']' => depth -= 1,
            b'0'..=b'9' => snailfish.push((c - b'0', depth)),
            _ => (),
        }
    }
    snailfish
}

fn reduce(mut sfnum: Vec<(u8, u8)>, split_allowed: bool) -> Vec<(u8, u8)> {
    let mut reduced: Vec<(u8, u8)> = Vec::with_capacity(sfnum.capacity());
    let mut i = 0;
    while i < sfnum.len() {
        let depth = sfnum[i].1;
        if depth > 4 {
            let (left, right) = (sfnum[i].0, sfnum[i + 1].0);
            if let Some(next) = sfnum.get_mut(i + 2) {
                next.0 += right;
            }
            sfnum[i + 1] = (0, depth - 1);
            if let Some(prev) = reduced.pop() {
                sfnum[i] = (prev.0 + left, prev.1);
            } else {
                i += 1;
            }
        } else if split_allowed && sfnum[i].0 > 9 {
            let left = sfnum[i].0 / 2;
            let right = sfnum[i].0 - left;
            if i == 0 {
                sfnum.insert(0, (left, depth + 1));
            } else {
                sfnum[i - 1] = (left, depth + 1);
                i -= 1;
            }
            sfnum[i + 1] = (right, depth + 1);
        } else {
            reduced.push(sfnum[i]);
            i += 1;
        }
    }
    reduced
}

fn add(mut this: Vec<(u8, u8)>, that: &[(u8, u8)]) -> Vec<(u8, u8)> {
    this.extend(that);
    if this.len() != that.len() {
        for num in this.iter_mut() {
            num.1 += 1;
        }
    }
    reduce(reduce(this, false), true)
}

fn magnitude(sfnum: &[(u8, u8)]) -> u32 {
    let mut mag = Vec::with_capacity(sfnum.len());
    for &(num, depth) in sfnum {
        mag.push((num as _, depth));
        while let Some(&[left, right]) = mag.get(mag.len().saturating_sub(2)..) {
            if left.1 != right.1 {
                break;
            }
            mag.truncate(mag.len() - 2);
            mag.push((3 * left.0 + 2 * right.0, left.1 - 1));
        }
    }
    mag[0].0
}

pub fn part1(input: &str) -> u32 {
    let sum = input
        .split_ascii_whitespace()
        .map(parse_snailfish)
        .fold(Vec::with_capacity(1 << 5), |acc, sfnum| add(acc, &sfnum));
    magnitude(&sum)
}

pub fn part2(input: &str) -> u32 {
    let sfnums: Vec<_> = input
        .split_ascii_whitespace()
        .map(parse_snailfish)
        .collect();
    let mut max_magnitude = 0;
    for i in 0..sfnums.len() {
        for j in 0..sfnums.len() {
            if i == j {
                continue;
            }
            let sum = add(sfnums[i].clone(), &sfnums[j]);
            max_magnitude = max_magnitude.max(magnitude(&sum));
        }
    }
    max_magnitude
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day18.txt").unwrap()),
        3806
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day18.txt").unwrap()),
        4727
    );
}
