// TODO: Rewrite Part 2 to line-scanned union-find.
//   Turns out span filling is not significantly faster than DFS...

struct HeightMap<'a> {
    map: std::borrow::Cow<'a, [u8]>,
    row: isize,
    col: isize,
}
impl<'a> HeightMap<'a> {
    fn new(s: &'a str) -> Self {
        let col = s.find('\n').unwrap() as isize;
        let row = s.len() as isize / (col + 1);
        let map = s.as_bytes().into();
        Self { map, row, col }
    }

    fn get(&self, row: isize, col: isize) -> u8 {
        if row < 0 || row >= self.row || col < 0 || col >= self.col {
            b'9'
        } else {
            self.map[(row * (self.col + 1) + col) as usize]
        }
    }

    fn get_left(&self, row: isize, col: isize) -> u8 {
        if col <= 0 {
            b'9'
        } else {
            self.map[(row * (self.col + 1) + col - 1) as usize]
        }
    }

    fn get_right(&self, row: isize, col: isize) -> u8 {
        if col >= self.col - 1 {
            b'9'
        } else {
            self.map[(row * (self.col + 1) + col + 1) as usize]
        }
    }

    fn get_up(&self, row: isize, col: isize) -> u8 {
        if row <= 0 {
            b'9'
        } else {
            self.map[((row - 1) * (self.col + 1) + col) as usize]
        }
    }

    fn get_down(&self, row: isize, col: isize) -> u8 {
        if row >= self.row - 1 {
            b'9'
        } else {
            self.map[((row + 1) * (self.col + 1) + col) as usize]
        }
    }

    fn set(&mut self, row: isize, col: isize) {
        self.map.to_mut()[(row * (self.col + 1) + col) as usize] = b'9'
    }

    fn fill(&mut self, row: isize, col: isize) -> u32 {
        let mut spans = Vec::with_capacity(1 << 4);
        spans.push((row, col, col + 1, 1));
        spans.push((row - 1, col, col + 1, -1));
        let mut count = 0;
        while let Some((row, mut col_lo, col_hi, drow)) = spans.pop() {
            if col_lo == col_hi {
                continue;
            }
            let mut col = col_lo;
            if self.get(row, col) != b'9' {
                while self.get_left(row, col) != b'9' {
                    self.set(row, col - 1);
                    count += 1;
                    col -= 1;
                }
            }
            if col < col_lo {
                spans.push((row - drow, col, col_lo, -drow));
            }
            while col_lo < col_hi {
                while self.get(row, col_lo) != b'9' {
                    self.set(row, col_lo);
                    count += 1;
                    col_lo += 1;
                }
                spans.push((row + drow, col, col_lo, drow));
                if col_lo > col_hi {
                    spans.push((row - drow, col_hi, col_lo, -drow));
                }
                while col_lo < col_hi && self.get(row, col_lo) == b'9' {
                    col_lo += 1;
                }
                col = col_lo;
            }
        }
        count
    }
}

pub fn part1(input: &str) -> u32 {
    let map = HeightMap::new(input);
    let mut sum = 0;
    for i in 0..map.row {
        for j in 0..map.col {
            let h = map.get(i, j);
            if h < map.get_left(i, j)
                && h < map.get_right(i, j)
                && h < map.get_up(i, j)
                && h < map.get_down(i, j)
            {
                sum += (h - b'0') as u32 + 1;
            }
        }
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut map = HeightMap::new(input);
    let mut seeds = Vec::new();
    for i in 0..map.row {
        for j in 0..map.col {
            let h = map.get(i, j);
            if h < map.get_left(i, j)
                && h < map.get_right(i, j)
                && h < map.get_up(i, j)
                && h < map.get_down(i, j)
            {
                seeds.push((i, j));
            }
        }
    }
    let mut sizes = std::collections::BinaryHeap::<u32>::new();
    while let Some((i, j)) = seeds.pop() {
        let size = map.fill(i, j);
        sizes.push(size);
    }
    sizes.pop().unwrap() * sizes.pop().unwrap() * sizes.pop().unwrap()
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day09.txt").unwrap()),
        603
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day09.txt").unwrap()),
        786780
    );
}
