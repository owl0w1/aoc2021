#[allow(clippy::type_complexity)]
fn parse_input(input: &str) -> (Vec<(usize, usize)>, Vec<(u8, usize)>) {
    let (dots_input, instrs_input) = input.split_once("\n\n").unwrap();
    let mut dots = Vec::with_capacity(1 << 10);
    for dot in dots_input.lines() {
        let (x, y) = dot.split_once(',').unwrap();
        dots.push((x.parse().unwrap(), y.parse().unwrap()));
    }
    let mut instrs = Vec::with_capacity(1 << 4);
    for instr in instrs_input.lines() {
        let dir = instr.as_bytes()[11];
        let axis = instr[13..].parse().unwrap();
        instrs.push((dir, axis));
    }
    (dots, instrs)
}

fn fold(dots: &[(usize, usize)], instrs: &[(u8, usize)]) -> (Vec<(usize, usize)>, usize, usize) {
    let mut folded_dots = Vec::with_capacity(1 << 9);
    let (mut min_x_axis, mut min_y_axis) = (usize::MAX, usize::MAX);
    for (mut x, mut y) in dots {
        for (dir, axis) in instrs {
            if *dir == b'x' {
                x = x.min(2 * axis - x);
                min_x_axis = *axis;
            } else {
                y = y.min(2 * axis - y);
                min_y_axis = *axis;
            }
        }
        folded_dots.push((x, y));
    }
    folded_dots.sort_unstable();
    folded_dots.dedup();
    (folded_dots, min_x_axis, min_y_axis)
}

pub fn part1(input: &str) -> u32 {
    let (dots, instrs) = parse_input(input);
    fold(&dots, &instrs[0..1]).0.len() as _
}

pub fn part2(input: &str) -> String {
    let (dots, instrs) = parse_input(input);
    let (folded_dots, canvas_width, canvas_height) = fold(&dots, &instrs);
    let mut canvas = vec![b'.'; (canvas_width + 1) * canvas_height - 1];
    for i in 0..(canvas_height - 1) {
        canvas[(canvas_width + 1) * i + canvas_width] = b'\n';
    }
    for (x, y) in folded_dots {
        canvas[(canvas_width + 1) * y + x] = b'#';
    }
    core::str::from_utf8(&canvas).unwrap().to_owned()
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day13.txt").unwrap()),
        708
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day13.txt").unwrap()),
        "####.###..#....#..#.###..###..####.#..#.\n\
         #....#..#.#....#..#.#..#.#..#.#....#..#.\n\
         ###..###..#....#..#.###..#..#.###..####.\n\
         #....#..#.#....#..#.#..#.###..#....#..#.\n\
         #....#..#.#....#..#.#..#.#.#..#....#..#.\n\
         ####.###..####..##..###..#..#.#....#..#."
    );
}
