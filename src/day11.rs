const GRID_SIZE: usize = 10;

fn parse(s: &str) -> [[u8; GRID_SIZE + 2]; GRID_SIZE + 2] {
    let nums = s.as_bytes().iter().filter(|c| **c != b'\n');
    let mut grid = [[0; GRID_SIZE + 2]; GRID_SIZE + 2];
    for (idx, num) in nums.enumerate() {
        grid[idx / GRID_SIZE + 1][idx % GRID_SIZE + 1] = *num - b'0';
    }
    grid
}

fn step(grid: &mut [[u8; GRID_SIZE + 2]; GRID_SIZE + 2]) -> u32 {
    #[allow(clippy::needless_range_loop)]
    for i in 1..=GRID_SIZE {
        for j in 1..=GRID_SIZE {
            grid[i][j] += 1;
        }
    }
    let mut count = 0;
    loop {
        let mut flashed = Vec::with_capacity(1 << 5);
        #[allow(clippy::needless_range_loop)]
        for i in 1..=GRID_SIZE {
            for j in 1..=GRID_SIZE {
                if grid[i][j] > 9 {
                    flashed.push((i, j));
                    if grid[i][j + 1] != 0 {
                        grid[i][j + 1] += 1;
                    }
                    if grid[i - 1][j + 1] != 0 {
                        grid[i - 1][j + 1] += 1;
                    }
                    if grid[i - 1][j] != 0 {
                        grid[i - 1][j] += 1;
                    }
                    if grid[i - 1][j - 1] != 0 {
                        grid[i - 1][j - 1] += 1;
                    }
                    if grid[i][j - 1] != 0 {
                        grid[i][j - 1] += 1;
                    }
                    if grid[i + 1][j - 1] != 0 {
                        grid[i + 1][j - 1] += 1;
                    }
                    if grid[i + 1][j] != 0 {
                        grid[i + 1][j] += 1;
                    }
                    if grid[i + 1][j + 1] != 0 {
                        grid[i + 1][j + 1] += 1;
                    }
                }
            }
        }
        if flashed.is_empty() {
            break;
        }
        count += flashed.len();
        for (i, j) in flashed {
            grid[i][j] = 0;
        }
    }
    count as u32
}

pub fn part1(input: &str) -> u32 {
    let mut grid = parse(input);
    let mut sum = 0;
    for _ in 0..100 {
        sum += step(&mut grid);
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut grid = parse(input);
    let mut step_num = 0;
    loop {
        step_num += 1;
        if step(&mut grid) == (GRID_SIZE * GRID_SIZE) as u32 {
            return step_num;
        }
    }
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day11.txt").unwrap()),
        1655
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day11.txt").unwrap()),
        337
    );
}
