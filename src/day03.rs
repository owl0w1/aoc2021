pub fn part1(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let mut one_counts = vec![0; width];
    let lines = input.as_bytes()[..input.len() - 1].split(|c| *c == b'\n');
    let mut line_count = 0;
    for num in lines {
        line_count += 1;
        for (idx, bit) in num.iter().enumerate() {
            one_counts[idx] += (*bit == b'1') as u32;
        }
    }
    let gamma = one_counts
        .into_iter()
        .fold(0, |n, count| (n << 1) | (count > line_count / 2) as u32);
    gamma * (!gamma & ((1 << width) - 1))
}

fn bit_majority(mut nums: &[u32], width: usize, flip: bool) -> u32 {
    let mut criteria_bit = 1 << width;
    let mut criteria = 0;
    while nums.len() > 1 {
        criteria_bit >>= 1;
        criteria |= criteria_bit;
        let partition_idx = nums.binary_search(&criteria).unwrap_or_else(|i| i);
        if flip ^ (partition_idx <= nums.len() / 2) {
            nums = &nums[partition_idx..];
        } else {
            nums = &nums[..partition_idx];
            criteria ^= criteria_bit;
        }
    }
    nums[0]
}

pub fn part2(input: &str) -> u32 {
    let width = input.find('\n').unwrap();
    let mut nums: Vec<_> = input.as_bytes()[..input.len() - 1]
        .split(|c| *c == b'\n')
        .map(|s| s.iter().fold(0, |n, bit| (n << 1) | (*bit == b'1') as u32))
        .collect();
    nums.sort_unstable();
    bit_majority(&nums, width, false) * bit_majority(&nums, width, true)
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day03.txt").unwrap()),
        2967914
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day03.txt").unwrap()),
        7041258
    );
}
