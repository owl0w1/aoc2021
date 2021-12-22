fn parse_input(input: &str) -> (i32, i32, i32, i32) {
    let mut nums = input[15..]
        .split(|c| !matches!(c, '0'..='9' | '-'))
        .filter_map(|s| s.parse().ok());
    let left = nums.next().unwrap();
    let right = nums.next().unwrap();
    let bottom = nums.next().unwrap();
    let top = nums.next().unwrap();
    (left, right, bottom, top)
}

fn test((left, right, bottom, top): (i32, i32, i32, i32), (mut vx, mut vy): (i32, i32)) -> bool {
    let (mut x, mut y) = (0, 0);
    while y >= bottom {
        if left <= x && x <= right && bottom <= y && y <= top {
            return true;
        }
        x += vx;
        y += vy;
        vx = 0.max(vx - 1);
        vy -= 1;
    }
    false
}

pub fn part1(input: &str) -> u32 {
    let (left, right, bottom, top) = parse_input(input);
    let vx_min = (0.5 * ((1 + 8 * left) as f32).sqrt() - 0.5).floor() as _;
    let vx_max = right;
    let vy_min = bottom;
    let vy_max = -bottom - 1;
    for vy in (vy_min..=vy_max).rev() {
        for vx in vx_min..=vx_max {
            if test((left, right, bottom, top), (vx, vy)) {
                return (vy * (vy + 1) / 2) as _;
            }
        }
    }
    panic!("invalid input");
}

pub fn part2(input: &str) -> u32 {
    let (left, right, bottom, top) = parse_input(input);
    let vx_min = (0.5 * ((1 + 8 * left) as f32).sqrt() - 0.5).floor() as _;
    let vx_max = right;
    let vy_min = bottom;
    let vy_max = -bottom - 1;
    let mut count = 0;
    for vx in vx_min..=vx_max {
        for vy in vy_min..=vy_max {
            if test((left, right, bottom, top), (vx, vy)) {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day17.txt").unwrap()),
        4186
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day17.txt").unwrap()),
        2709
    );
}
