struct Grid<'a> {
    string: &'a [u8],
    row: usize,
    col: usize,
    repeat: usize,
}
impl<'a> Grid<'a> {
    fn new(s: &'a str, repeat: usize) -> Self {
        let col = s.find('\n').unwrap();
        let row = s.len() / (col + 1);
        Self {
            string: s.as_bytes(),
            row,
            col,
            repeat,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<u8> {
        if 0 < row && row <= self.row * self.repeat && 0 < col && col <= self.col * self.repeat {
            let (row, col) = (row - 1, col - 1);
            let inc = (row / self.row + col / self.col) as u8;
            let (row, col) = (row % self.row, col % self.col);
            Some((self.string[(self.col + 1) * row + col] - b'0' + inc - 1) % 9 + 1)
        } else {
            None
        }
    }

    fn shortest_path(&self, target_row: usize, target_col: usize) -> u32 {
        use std::{cmp::Reverse, collections::BinaryHeap};
        let mut fringe = BinaryHeap::with_capacity(1 << 10);
        fringe.push((Reverse(0), (1, 1)));
        let mut dist = vec![u32::MAX; self.row * self.col * self.repeat * self.repeat];
        loop {
            let (Reverse(d), (i, j)) = fringe.pop().unwrap();
            if d > dist[self.col * self.repeat * (i - 1) + j - 1] {
                continue;
            }
            if (i, j) == (target_row, target_col) {
                return d;
            }
            for (ni, nj) in [(i, j - 1), (i, j + 1), (i - 1, j), (i + 1, j)] {
                if let Some(edge) = self.get(ni, nj) {
                    let nd = d + edge as u32;
                    if nd < dist[self.col * self.repeat * (ni - 1) + nj - 1] {
                        dist[self.col * self.repeat * (ni - 1) + nj - 1] = nd;
                        fringe.push((Reverse(nd), (ni, nj)));
                    }
                }
            }
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let grid = Grid::new(input, 1);
    grid.shortest_path(grid.row, grid.col)
}

pub fn part2(input: &str) -> u32 {
    let repeat = 5;
    let grid = Grid::new(input, repeat);
    grid.shortest_path(grid.row * repeat, grid.col * repeat)
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day15.txt").unwrap()),
        462
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day15.txt").unwrap()),
        2846
    );
}
