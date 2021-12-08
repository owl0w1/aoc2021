const BOARD_SIZE: usize = 5;

struct Board {
    board: [[u32; BOARD_SIZE]; BOARD_SIZE],
    mark_turn: [[usize; BOARD_SIZE]; BOARD_SIZE],
    win_turn: usize,
}
impl Board {
    fn new(s: &str) -> Self {
        let nums = s.split_ascii_whitespace().filter_map(|s| s.parse().ok());
        let mut board = [[0; BOARD_SIZE]; BOARD_SIZE];
        for (idx, num) in nums.enumerate() {
            board[idx / BOARD_SIZE][idx % BOARD_SIZE] = num;
        }
        Self {
            board,
            mark_turn: [[usize::MAX; BOARD_SIZE]; BOARD_SIZE],
            win_turn: usize::MAX,
        }
    }

    fn mark(&mut self, nums: &std::collections::HashMap<u32, usize>) -> usize {
        let mut row_win_turn = [0; BOARD_SIZE];
        let mut col_win_turn = [0; BOARD_SIZE];
        #[allow(clippy::needless_range_loop)]
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                self.mark_turn[i][j] = *nums.get(&self.board[i][j]).unwrap_or(&usize::MAX);
                row_win_turn[i] = row_win_turn[i].max(self.mark_turn[i][j]);
                col_win_turn[j] = col_win_turn[j].max(self.mark_turn[i][j]);
            }
        }
        self.win_turn = usize::min(
            row_win_turn.into_iter().min().unwrap(),
            col_win_turn.into_iter().min().unwrap(),
        );
        self.win_turn
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                if self.mark_turn[i][j] > self.win_turn {
                    sum += self.board[i][j];
                }
            }
        }
        sum
    }
}

fn parse_input(input: &str) -> (Vec<u32>, Vec<Board>) {
    let (nums_input, boards_input) = input.split_once('\n').unwrap();
    let nums = nums_input.split(',').map(|s| s.parse().unwrap()).collect();
    let boards = boards_input[2..].split("\n\n").map(Board::new).collect();
    (nums, boards)
}

pub fn part1(input: &str) -> u32 {
    let (nums, mut boards) = parse_input(input);
    let nums_map = nums
        .iter()
        .enumerate()
        .map(|(idx, num)| (*num, idx))
        .collect();
    let mut first_win_turn = usize::MAX;
    let mut first_win_board = 0;
    for (idx, board) in boards.iter_mut().enumerate() {
        let win_turn = board.mark(&nums_map);
        if first_win_turn > win_turn {
            first_win_turn = win_turn;
            first_win_board = idx;
        }
    }
    nums[first_win_turn] * boards[first_win_board].unmarked_sum()
}

pub fn part2(input: &str) -> u32 {
    let (nums, mut boards) = parse_input(input);
    let nums_map = nums
        .iter()
        .enumerate()
        .map(|(idx, num)| (*num, idx))
        .collect();
    let mut last_win_turn = 0;
    let mut last_win_board = 0;
    for (idx, board) in boards.iter_mut().enumerate() {
        let win_turn = board.mark(&nums_map);
        if last_win_turn <= win_turn {
            last_win_turn = win_turn;
            last_win_board = idx;
        }
    }
    nums[last_win_turn] * boards[last_win_board].unmarked_sum()
}

#[cfg(test)]
#[test]
fn part1_test() {
    assert_eq!(
        part1(&std::fs::read_to_string("input/day04.txt").unwrap()),
        27027
    );
}
#[cfg(test)]
#[test]
fn part2_test() {
    assert_eq!(
        part2(&std::fs::read_to_string("input/day04.txt").unwrap()),
        36975
    );
}
