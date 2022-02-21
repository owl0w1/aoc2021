fn parse_input(input: &str) -> (u8, u8) {
    let input = input.as_bytes();
    let p1 = if input[29] == b'0' {
        10
    } else {
        input[28] - b'0'
    };
    let p2 = if input[input.len() - 2] == b'0' {
        10
    } else {
        input[input.len() - 2] - b'0'
    };
    (p1 - 1, p2 - 1)
}

pub fn part1(input: &str) -> u32 {
    let (mut p1, mut p2) = {
        let p = parse_input(input);
        (p.0 as u32, p.1 as u32)
    };
    let (mut s1, mut s2) = (0, 0);
    let mut dice = 0;
    let mut dice_count = 0;
    loop {
        let step = dice + (dice + 1) % 100 + (dice + 2) % 100 + 3;
        dice = (dice + 3) % 100;
        dice_count += 3;
        p1 = (p1 + step) % 10;
        s1 += p1 + 1;
        if s1 >= 1000 {
            break s2 * dice_count;
        }
        let step = dice + (dice + 1) % 100 + (dice + 2) % 100 + 3;
        dice = (dice + 3) % 100;
        dice_count += 3;
        p2 = (p2 + step) % 10;
        s2 += p2 + 1;
        if s2 >= 1000 {
            break s1 * dice_count;
        }
    }
}

#[allow(clippy::type_complexity)]
fn count_win(
    cache: &mut [[[[(u64, u64); 22]; 10]; 22]; 10],
    p1: usize,
    s1: usize,
    p2: usize,
    s2: usize,
) -> (u64, u64) {
    if s1 >= 21 {
        return (1, 0);
    }
    if s2 >= 21 {
        return (0, 1);
    }
    if cache[p1][s1][p2][s2] != (0, 0) {
        return cache[p1][s1][p2][s2];
    }
    let mut win_count = (0, 0);
    for d1 in [1, 2, 3] {
        for d2 in [1, 2, 3] {
            for d3 in [1, 2, 3] {
                let p1_next = (p1 + d1 + d2 + d3) % 10;
                let s1_next = s1 + p1_next + 1;
                let (win2, win1) = count_win(cache, p2, s2, p1_next, s1_next);
                win_count = (win_count.0 + win1, win_count.1 + win2);
            }
        }
    }
    cache[p1][s1][p2][s2] = win_count;
    win_count
}

pub fn part2(input: &str) -> u64 {
    let (p1, p2) = parse_input(input);
    let mut cache = [[[[(0, 0); 22]; 10]; 22]; 10];
    let (win1, win2) = count_win(&mut cache, p1 as _, 0, p2 as _, 0);
    win1.max(win2)
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day21.txt").unwrap()),
        1196172
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day21.txt").unwrap()),
        106768284484217
    );
}
