// TODO: Rewrite. This is terrible code.
//   Simple version: Turn each line into an iterator producing points. Use HashMap to count overlap.
//   Overkill version: "AABB" Project each line onto a number on corresponding axis (e.g. horizontal
//     lines project to y-axis; diagonal up-right lines project to y=-x). Sort these projections,
//     use binary search to filter possible collisions.

fn parse_line(line: &str) -> ((u32, u32), (u32, u32)) {
    let mut nums = line
        .split(&[',', ' ', '-', '>'][..])
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap());
    let x1 = nums.next().unwrap();
    let y1 = nums.next().unwrap();
    let x2 = nums.next().unwrap();
    let y2 = nums.next().unwrap();
    ((x1, y1), (x2, y2))
}

pub fn part1(input: &str) -> u32 {
    let lines = input.lines();
    let mut count = 0;
    let mut covered_pos = std::collections::HashMap::<_, bool>::with_capacity(128);
    for (p1, p2) in lines.map(parse_line) {
        if p1.0 == p2.0 {
            let ys = if p1.1 < p2.1 {
                p1.1..=p2.1
            } else {
                p2.1..=p1.1
            };
            for y in ys {
                if let Some(twice_covered) = covered_pos.get_mut(&(p1.0, y)) {
                    if !*twice_covered {
                        count += 1;
                        *twice_covered = true;
                    }
                } else {
                    covered_pos.insert((p1.0, y), false);
                }
            }
        }
        if p1.1 == p2.1 {
            let xs = if p1.0 < p2.0 {
                p1.0..=p2.0
            } else {
                p2.0..=p1.0
            };
            for x in xs {
                if let Some(twice_covered) = covered_pos.get_mut(&(x, p1.1)) {
                    if !*twice_covered {
                        count += 1;
                        *twice_covered = true;
                    }
                } else {
                    covered_pos.insert((x, p1.1), false);
                }
            }
        }
    }
    count
}

pub fn part2(input: &str) -> u32 {
    let lines = input.lines();
    let mut count = 0;
    let mut covered_pos = std::collections::HashMap::<_, bool>::with_capacity(128);
    for (mut p1, mut p2) in lines.map(parse_line) {
        if p1.0 == p2.0 {
            let ys = if p1.1 < p2.1 {
                p1.1..=p2.1
            } else {
                p2.1..=p1.1
            };
            for y in ys {
                if let Some(twice_covered) = covered_pos.get_mut(&(p1.0, y)) {
                    if !*twice_covered {
                        count += 1;
                        *twice_covered = true;
                    }
                } else {
                    covered_pos.insert((p1.0, y), false);
                }
            }
        } else if p1.1 == p2.1 {
            let xs = if p1.0 < p2.0 {
                p1.0..=p2.0
            } else {
                p2.0..=p1.0
            };
            for x in xs {
                if let Some(twice_covered) = covered_pos.get_mut(&(x, p1.1)) {
                    if !*twice_covered {
                        count += 1;
                        *twice_covered = true;
                    }
                } else {
                    covered_pos.insert((x, p1.1), false);
                }
            }
        } else {
            if p1.0 > p2.0 {
                std::mem::swap(&mut p1, &mut p2);
            }
            #[allow(clippy::comparison_chain)]
            if p1.1 < p2.1 {
                for delta in 0..=(p2.1 - p1.1) {
                    if let Some(twice_covered) = covered_pos.get_mut(&(p1.0 + delta, p1.1 + delta))
                    {
                        if !*twice_covered {
                            count += 1;
                            *twice_covered = true;
                        }
                    } else {
                        covered_pos.insert((p1.0 + delta, p1.1 + delta), false);
                    }
                }
            } else if p1.1 > p2.1 {
                for delta in 0..=(p1.1 - p2.1) {
                    if let Some(twice_covered) = covered_pos.get_mut(&(p1.0 + delta, p1.1 - delta))
                    {
                        if !*twice_covered {
                            count += 1;
                            *twice_covered = true;
                        }
                    } else {
                        covered_pos.insert((p1.0 + delta, p1.1 - delta), false);
                    }
                }
            }
        }
    }
    count
}

// fn parse_line(s: &str) -> impl Iterator<Item = (u32, u32)> {
//     let mut nums = s
//         .split(|c: char| !c.is_ascii_digit())
//         .filter_map(|s| s.parse().ok());
//     let mut x1 = nums.next().unwrap();
//     let mut y1 = nums.next().unwrap();
//     let mut x2 = nums.next().unwrap();
//     let mut y2 = nums.next().unwrap();
//     if x1 > x2 || (x1 == x2 && y1 > y2) {
//         std::mem::swap(&mut x1, &mut x2);
//         std::mem::swap(&mut y1, &mut y2);
//     }
//     use std::iter::from_fn;
//     if x1 == x2 {
//         // Up
//         (y1..=y2).map(|y| (x1, y))
//     } else if y1 == y2 {
//         // Right
//         (y1..=y2).map(|y| (x1, y))
//     } else if y1 < y2 {
//         // RightUp
//         (y1..=y2).map(|y| (x1, y))
//     } else {
//         // RightDown
//         (y1..=y2).map(|y| (x1, y))
//     }
// }

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day05.txt").unwrap()),
        7414
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day05.txt").unwrap()),
        19676
    );
}
