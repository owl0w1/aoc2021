pub fn part1(input: &str) -> u32 {
    let digits = input.lines().flat_map(|s| s[61..].split_ascii_whitespace());
    digits.filter(|s| matches!(s.len(), 2 | 4 | 3 | 7)).count() as _
}

fn parse_seven_seg(s: &str) -> u8 {
    s.bytes().fold(0, |n, seg| n | (1 << (seg - b'a')))
}

pub fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let clues = line[..58].split_ascii_whitespace();
        let mut seg_one = 0;
        let mut seg_four = 0;
        for clue in clues {
            match clue.len() {
                2 => seg_one = parse_seven_seg(clue),
                4 => seg_four = parse_seven_seg(clue),
                _ => (),
            }
            if seg_one != 0 && seg_four != 0 {
                break;
            }
        }
        let seg_bd = seg_four ^ seg_one;
        let digits = line[61..].split_ascii_whitespace();
        let mut val = 0;
        for digit in digits {
            val *= 10;
            val += match digit.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                5 => {
                    let seg = parse_seven_seg(digit);
                    if seg | seg_one == seg {
                        3
                    } else if seg | seg_bd == seg {
                        5
                    } else {
                        2
                    }
                }
                6 => {
                    let seg = parse_seven_seg(digit);
                    if seg | seg_four == seg {
                        9
                    } else if seg | seg_bd == seg {
                        6
                    } else {
                        0
                    }
                }
                _ => panic!("invalid input"),
            };
        }
        sum += val;
    }
    sum
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day08.txt").unwrap()),
        476
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day08.txt").unwrap()),
        1011823
    );
}
