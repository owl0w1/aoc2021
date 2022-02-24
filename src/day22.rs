type Cuboid = (i64, (i64, i64, i64), (i64, i64, i64));

fn parse_line(line: &str) -> Cuboid {
    let sign = if line.as_bytes()[1] == b'n' { 1 } else { -1 };
    let mut nums = line
        .split(|c| !matches!(c, '0'..='9' | '-'))
        .filter_map(|s| s.parse().ok());
    let x1 = nums.next().unwrap();
    let x2 = nums.next().unwrap();
    let y1 = nums.next().unwrap();
    let y2 = nums.next().unwrap();
    let z1 = nums.next().unwrap();
    let z2 = nums.next().unwrap();
    (sign, (x1, y1, z1), (x2, y2, z2))
}

fn count_on(cuboids: impl Iterator<Item = Cuboid>) -> u64 {
    let mut signed_cuboids: Vec<(i64, _, _)> = Vec::with_capacity(1 << 14);
    let mut removed = 0;
    for (sign, p, q) in cuboids {
        let ((x1, y1, z1), (x2, y2, z2)) = (p, q);
        #[allow(clippy::mut_range_bound)]
        for i in removed..signed_cuboids.len() {
            let (other_sign, other_p, other_q) = signed_cuboids[i];
            let ((ox1, oy1, oz1), (ox2, oy2, oz2)) = (other_p, other_q);
            let (ix1, iy1, iz1) = (x1.max(ox1), y1.max(oy1), z1.max(oz1));
            let (ix2, iy2, iz2) = (x2.min(ox2), y2.min(oy2), z2.min(oz2));
            let (overlap_p, overlap_q) = ((ix1, iy1, iz1), (ix2, iy2, iz2));
            if ix1 <= ix2 && iy1 <= iy2 && iz1 <= iz2 {
                if overlap_p == other_p && overlap_q == other_q {
                    signed_cuboids.swap(i, removed);
                    removed += 1;
                } else {
                    signed_cuboids.push((-other_sign, overlap_p, overlap_q));
                }
            }
        }
        if sign == 1 {
            signed_cuboids.push((sign, p, q));
        }
    }
    signed_cuboids[removed..]
        .iter()
        .map(|(sign, (x1, y1, z1), (x2, y2, z2))| {
            sign * (z2 - z1 + 1) * (y2 - y1 + 1) * (x2 - x1 + 1)
        })
        .sum::<i64>() as _
}

pub fn part1(input: &str) -> u64 {
    let bottom_left = |(x, y, z)| x >= -50 && y >= -50 && z >= -50;
    let top_right = |(x, y, z)| x <= 50 && y <= 50 && z <= 50;
    count_on(
        input
            .lines()
            .map(parse_line)
            .filter(|(_, p, q)| bottom_left(*p) && top_right(*q)),
    )
}

pub fn part2(input: &str) -> u64 {
    count_on(input.lines().map(parse_line))
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day22.txt").unwrap()),
        524792
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day22.txt").unwrap()),
        1213461324555691
    );
}
