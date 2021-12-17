const GRID_SIZE: usize = 10;

fn parse_input(input: &str) -> [[u8; GRID_SIZE + 2]; GRID_SIZE + 2] {
    let nums = input.as_bytes().iter().filter(|c| **c != b'\n');
    let mut grid = [[0; GRID_SIZE + 2]; GRID_SIZE + 2];
    for (idx, num) in nums.enumerate() {
        grid[idx / GRID_SIZE + 1][idx % GRID_SIZE + 1] = num - b'0';
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
    let mut flashed = true;
    while flashed {
        flashed = false;
        #[allow(clippy::needless_range_loop)]
        for i in 1..=GRID_SIZE {
            for j in 1..=GRID_SIZE {
                if grid[i][j] > 9 {
                    grid[i][j] = 0;
                    flashed = true;
                    count += 1;
                    grid[i][j + 1] += (grid[i][j + 1] != 0) as u8;
                    grid[i - 1][j + 1] += (grid[i - 1][j + 1] != 0) as u8;
                    grid[i - 1][j] += (grid[i - 1][j] != 0) as u8;
                    grid[i - 1][j - 1] += (grid[i - 1][j - 1] != 0) as u8;
                    grid[i][j - 1] += (grid[i][j - 1] != 0) as u8;
                    grid[i + 1][j - 1] += (grid[i + 1][j - 1] != 0) as u8;
                    grid[i + 1][j] += (grid[i + 1][j] != 0) as u8;
                    grid[i + 1][j + 1] += (grid[i + 1][j + 1] != 0) as u8;
                }
            }
        }
    }
    count
}

pub fn part1(input: &str) -> u32 {
    let mut grid = parse_input(input);
    let mut sum = 0;
    for _ in 0..100 {
        sum += step(&mut grid);
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut grid = parse_input(input);
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
