// TODO: Current brute-force method is very slow (2.7s in release).
//   Rewrite, use signature to speedup finding the origin points pair
//   and correct rotation.

fn parse_input(input: &str) -> Vec<Vec<(i32, i32, i32)>> {
    let mut scanners = Vec::with_capacity(1 << 6);
    let input_sections = input.split("\n\n");
    for s in input_sections {
        let mut points = Vec::with_capacity(1 << 5);
        for p in s.lines().skip(1) {
            let mut coords = p.split(',');
            let x = coords.next().unwrap().parse().unwrap();
            let y = coords.next().unwrap().parse().unwrap();
            let z = coords.next().unwrap().parse().unwrap();
            points.push((x, y, z));
        }
        scanners.push(points);
    }
    scanners
}

fn rotate_point((x, y, z): (i32, i32, i32), rotation: u8) -> (i32, i32, i32) {
    match rotation {
        0 => (x, y, z),
        1 => (x, -y, -z),
        2 => (-x, -y, z),
        3 => (-x, y, -z),
        4 => (x, z, -y),
        5 => (x, -z, y),
        6 => (-x, -z, -y),
        7 => (-x, z, y),
        8 => (y, z, x),
        9 => (y, -z, -x),
        10 => (-y, -z, x),
        11 => (-y, z, -x),
        12 => (y, x, -z),
        13 => (y, -x, z),
        14 => (-y, -x, -z),
        15 => (-y, x, z),
        16 => (z, x, y),
        17 => (z, -x, -y),
        18 => (-z, -x, y),
        19 => (-z, x, -y),
        20 => (z, y, -x),
        21 => (z, -y, x),
        22 => (-z, -y, -x),
        23 => (-z, y, x),
        _ => panic!(),
    }
}

fn align_scan(
    ref_scan: &[(i32, i32, i32)],
    candidate_scan: &mut Vec<(i32, i32, i32)>,
) -> Option<(i32, i32, i32)> {
    for rotation in 0..24 {
        let rotated: Vec<_> = candidate_scan
            .iter()
            .map(|p| rotate_point(*p, rotation))
            .collect();
        for ref_origin in ref_scan {
            for candidate_origin in &rotated {
                let ((x1, y1, z1), (x2, y2, z2)) = (*ref_origin, *candidate_origin);
                let (dx, dy, dz) = (x1 - x2, y1 - y2, z1 - z2);
                let translated = rotated.iter().map(|(x, y, z)| (x + dx, y + dy, z + dz));
                let overlap_count = translated
                    .clone()
                    .filter(|p| ref_scan.binary_search(p).is_ok())
                    .count();
                if overlap_count >= 12 {
                    *candidate_scan = translated.collect();
                    candidate_scan.sort_unstable();
                    return Some((dx, dy, dz));
                }
            }
        }
    }
    None
}

fn reconstruct_scans(scans: &mut Vec<Vec<(i32, i32, i32)>>) -> Vec<(i32, i32, i32)> {
    scans[0].sort_unstable();
    let mut scanner_pos = vec![None; scans.len()];
    scanner_pos[0] = Some((0, 0, 0));
    let mut visited = vec![false; scans.len()];
    let mut known_scanner_count = 1;
    while known_scanner_count < scans.len() {
        let ref_scan_idx = (0..scans.len())
            .find(|i| scanner_pos[*i].is_some() && !visited[*i])
            .unwrap();
        visited[ref_scan_idx] = true;
        for candidate_scan_idx in 0..scans.len() {
            if scanner_pos[candidate_scan_idx].is_some() {
                continue;
            }
            let (ref_scan, candidate_scan) = if ref_scan_idx < candidate_scan_idx {
                let (fst, snd) = scans.split_at_mut(candidate_scan_idx);
                (&fst[ref_scan_idx], &mut snd[0])
            } else {
                let (fst, snd) = scans.split_at_mut(ref_scan_idx);
                (&snd[0], &mut fst[candidate_scan_idx])
            };
            if let Some(pos) = align_scan(ref_scan, candidate_scan) {
                scanner_pos[candidate_scan_idx] = Some(pos);
                known_scanner_count += 1;
            }
        }
    }
    scanner_pos.into_iter().map(|p| p.unwrap()).collect()
}

pub fn part1(input: &str) -> u32 {
    let mut scans = parse_input(input);
    reconstruct_scans(&mut scans);
    let mut beacons: Vec<_> = scans.into_iter().flat_map(|x| x.into_iter()).collect();
    beacons.sort_unstable();
    beacons.dedup();
    beacons.len() as _
}

pub fn part2(input: &str) -> u32 {
    let mut scans = parse_input(input);
    let scanner_pos = reconstruct_scans(&mut scans);
    let mut max_dist = 0;
    for p1 in &scanner_pos {
        for p2 in &scanner_pos {
            let dist = (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs() + (p1.2 - p2.2).abs();
            max_dist = max_dist.max(dist);
        }
    }
    max_dist as _
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day19.txt").unwrap()),
        457
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day19.txt").unwrap()),
        13243
    );
}
