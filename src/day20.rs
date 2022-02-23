fn parse_input(input: &str) -> ([bool; 512], Vec<bool>, usize) {
    let input = input.as_bytes();
    let (rule_str, image_str) = (&input[..512], &input[514..]);
    let mut rule = [false; 512];
    for i in 0..512 {
        rule[i] = rule_str[i] == b'#';
    }
    let image = image_str
        .iter()
        .filter_map(|c| if *c == b'\n' { None } else { Some(*c == b'#') })
        .collect();
    let col = image_str.iter().position(|c| *c == b'\n').unwrap();
    (rule, image, col)
}

fn enhance(rule: &[bool; 512], round: usize, image: Vec<bool>, col: usize) -> Vec<bool> {
    let row = image.len() / col;
    let (next_row, next_col) = (row + 2, col + 2);
    let mut next_image = Vec::with_capacity(next_row * next_col);
    for i in 1..=next_row {
        for j in 1..=next_col {
            let mut pattern = 0;
            for ni in (i - 1)..=(i + 1) {
                for nj in (j - 1)..=(j + 1) {
                    let pixel = if ni <= 1 || ni >= next_row || nj <= 1 || nj >= next_col {
                        (round % 2 != 0 || rule[511]) && rule[0]
                    } else {
                        image[col * (ni - 2) + nj - 2]
                    };
                    pattern = (pattern << 1) | pixel as usize;
                }
            }
            next_image.push(rule[pattern])
        }
    }
    next_image
}

pub fn part1(input: &str) -> u32 {
    let (rule, mut image, col) = parse_input(input);
    image = enhance(&rule, 0, image, col);
    image = enhance(&rule, 1, image, col + 2);
    image.into_iter().filter(|p| *p).count() as _
}

pub fn part2(input: &str) -> u32 {
    let (rule, mut image, col) = parse_input(input);
    for round in 0..50 {
        image = enhance(&rule, round, image, col + 2 * round);
    }
    image.into_iter().filter(|p| *p).count() as _
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day20.txt").unwrap()),
        5359
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day20.txt").unwrap()),
        12333
    );
}
