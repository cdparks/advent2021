use std::convert::TryInto;
use std::fmt::{self, Display, Formatter};

/// Find the first board that can win.
///
/// The result is the board score × the last number picked.
pub fn part1(input: &[String]) -> i32 {
    let picks = parse_numbers(&input[0]);
    let mut boards = parse_boards(&input[1..]);
    for n in picks {
        for board in boards.iter_mut() {
            if board.mark(n) {
                return board.score();
            }
        }
    }
    0
}

/// Find the last board that can win.
///
/// The result is the board score × the last number picked.
pub fn part2(input: &[String]) -> i32 {
    let picks = parse_numbers(&input[0]);
    let mut boards = parse_boards(&input[1..]);
    let mut last_score = 0;
    for n in picks {
        for board in boards.iter_mut() {
            if board.mark(n) {
                last_score = board.score();
            }
        }
    }
    last_score
}

/// Parse "randomly" chosen numbers.
pub fn parse_numbers(line: &String) -> Vec<i32> {
    line.split(',').flat_map(str::parse).collect()
}

/// Parse sequence of boards.
pub fn parse_boards(lines: &[String]) -> Vec<Board> {
    lines
        .chunks(6)
        .map(|board| {
            let board: Vec<String> = board.iter().skip(1).map(|x| x.clone()).collect();
            parse_board(&board)
        })
        .collect()
}

/// Parse a single board.
pub fn parse_board(lines: &[String]) -> Board {
    let squares = lines
        .iter()
        .flat_map(|line| line.split_whitespace())
        .flat_map(str::parse)
        .collect();
    Board::new(squares)
}

#[derive(PartialEq, Eq, Debug)]
/// A 5 × 5 Bingo board that tracks its own state
pub struct Board {
    /// Flat array of squares
    squares: [i32; 25],
    /// Bitmask indicating which squares are marked
    marked: u32,
    /// Value of last square to be marked
    last: i32,
    /// Whether this board has already won
    won: bool,
}

impl Board {
    /// Create a new board from a vector of 25 numbers.
    ///
    /// # Panics
    ///
    /// Panics if not passed exactly 25 numbers.
    pub fn new(squares: Vec<i32>) -> Self {
        let squares = squares.try_into().expect("25 elements");
        Self {
            squares,
            marked: 0,
            last: 0,
            won: false,
        }
    }

    /// Mark the square if this board has it and return whether this turn wins
    pub fn mark(&mut self, n: i32) -> bool {
        if self.won {
            return false;
        }

        let mut check = false;
        for (i, square) in self.squares.iter().rev().enumerate() {
            if *square == n {
                self.marked |= 1 << i;
                self.last = n;
                check = true;
            }
        }

        self.won = check
            && WINNING_MASKS
                .iter()
                .any(|mask| (mask & self.marked) == *mask);
        self.won
    }

    /// Calculate score as sum of unmarked numbers × last number marked
    pub fn score(&self) -> i32 {
        let unused: i32 = self
            .squares
            .iter()
            .rev()
            .enumerate()
            .filter_map(|(i, square)| {
                if self.marked & (1 << i) == 0 {
                    Some(*square)
                } else {
                    None
                }
            })
            .sum();
        self.last * unused
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for i in 0..5 {
            for j in 0..5 {
                if j == 0 {
                    write!(f, "{:>2}", self.squares[i * 5 + j])?;
                } else {
                    write!(f, ", {:>2}", self.squares[i * 5 + j])?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

/// Bit patterns of winning boards.
pub static WINNING_MASKS: [u32; 10] = [
    0b1111100000000000000000000, // row 1
    0b0000011111000000000000000, // row 2
    0b0000000000111110000000000, // row 3
    0b0000000000000001111100000, // row 4
    0b0000000000000000000011111, // row 5
    0b1000010000100001000010000, // col 1
    0b0100001000010000100001000, // col 2
    0b0010000100001000010000100, // col 3
    0b0001000010000100001000010, // col 4
    0b0000100001000010000100001, // col 5
];

check!(ex 1 = 4512, ex 2 = 1924, part 1 = 8442, part 2 = 4590);
bench!(part 1, part 2);
