pub fn part1(input: &str) -> u32 {
    let mut nums: Vec<i32> = input
        .split_terminator(&[',', '\n'][..])
        .map(|s| s.parse().unwrap())
        .collect();
    let nums_len = nums.len();
    nums.select_nth_unstable(nums_len / 2);
    nums.select_nth_unstable((nums_len + 1) / 2);
    let median = (nums[nums_len / 2] + nums[(nums_len + 1) / 2]) / 2;
    nums.into_iter()
        .map(|num| (num - median).abs() as u32)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let nums: Vec<i32> = input
        .split_terminator(&[',', '\n'][..])
        .map(|s| s.parse().unwrap())
        .collect();
    let mean_lower = nums.iter().sum::<i32>() / nums.len() as i32;
    let ssd0 = nums
        .iter()
        .map(|num| (num - mean_lower) * (num - mean_lower) + (num - mean_lower).abs())
        .sum::<i32>();
    let mean_upper = mean_lower + 1;
    let ssd1 = nums
        .iter()
        .map(|num| (num - mean_upper) * (num - mean_upper) + (num - mean_upper).abs())
        .sum::<i32>();
    ssd0.min(ssd1) as u32 / 2
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day07.txt").unwrap()),
        323647
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day07.txt").unwrap()),
        87640209
    );
}
