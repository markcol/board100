/*

A Rust solver for a simple numerical game.

See the inital article at [simple-number].

# Rules

The rules are simple: on an empty 10x10 grid (100 squares in total) you put a
number 1 on an arbitrary square. Starting from that square you can move
horizontally or vertically jumping over two squares or diagonally jumping over
one square. There you can place number 2. Your task is to reach number 100,
filling all squares. You can not visit already visited squares.

Here is an example of a solved game with a reduced 5x5 grid, starting at
top-left corner

     1 24 14  2 25
    16 21  5  8 20
    13 10 18 23 11
     4  7 15  3  6
    17 22 12  9 19

[simple-number]: https://www.nurkiewicz.com/2018/09/brute-forcing-seemingly-simple-number.html
 */

#![allow(dead_code)]

use std::slice::Iter;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct BoardError {
    details: String
}

impl BoardError{
    fn new(msg: &str) -> BoardError {
        BoardError{
            details: msg.to_string()
        }
    }
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for BoardError {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Distance from source for horizontal or vertical moves.
pub const HV_OFFSET: i32 = 3;

/// Distance from source for diagnal moves (both horizontal and vertical).
pub const DIAG_OFFSET: i32 = 2;

#[derive(Debug, Copy, Clone)]
/// Direction represents the direction of a move from the source location.
pub enum Direction {
    Down,
    DownRight,
    Right,
    UpRight,
    Up,
    UpLeft,
    Left,
    DownLeft,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Down,
            Direction::DownRight,
            Direction::Right,
            Direction::UpRight,
            Direction::Up,
            Direction::UpLeft,
            Direction::Left,
            Direction::DownLeft,
        ];
        DIRECTIONS.into_iter()
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Direction::Down => "Down".to_string(),
            Direction::DownRight => "Down Right".to_string(),
            Direction::Right => "Right".to_string(),
            Direction::UpRight => "Up Right".to_string(),
            Direction::Up => "Up".to_string(),
            Direction::UpLeft => "Up Left".to_string(),
            Direction::Left => "Left".to_string(),
            Direction::DownLeft => "Down Left".to_string(),
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
/// Board represents the puzzle board. It is a square grid of
/// values 0-(size x size), where size is the vertical/horizontal
/// dimensions of the board. O represents an empty cell.
pub struct Board {
    /// The number of vertical/horizontal cells in te board.
    size: usize,
    /// The total number of cells in the board (size x size).
    cells: usize,
    /// The values of the cell in the board.
    values: Vec<u8>,
    /// The x location of the last cell set in the board.
    x: usize,
    /// The y location of the last cell set in the board.
    y: usize,
}

impl Board {
    /// Create a new board with the dimensions `size` x `size`.
    pub fn new(size: usize) -> Self {
        let mut size = size;
        if size < 5 {
            size = 5;
        }
        if size > 16 {
            size = 16;
        }

        Board {
            size,
            cells: size * size,
            values: vec![0; size * size],
            x: 0,
            y: 0,
        }
    }

    /// Return `true` if the board has been started; otherwise `false`.
    pub fn is_started(&self) -> bool {
        self.value_at(self.x, self.y) > 0
    }

    /// Return a list of all possible moves from the current location.
    /// Returns an empty list if there are no moves, or the board is empty.
    pub fn possible_moves(&self) -> Vec<&'static Direction> {
        Direction::iterator().filter(|&x| self.valid_move(*x).is_some()).collect()
    }

    /// Determines if a move in the given direction is valid. A move is valid
    /// if the resulting position is valid, and if the the resulting position
    /// is an empty cell. If the move is valid, it returns `Some((x, y))` 
    /// where (x, y) is the cell location resulting from the move. Otherwise,
    /// it returns `None`.
    fn valid_move(&self, dir: Direction) -> Option<(usize, usize)> {
        let x: i32 = self.x as i32;
        let y: i32 = self.y as i32;
        let size: i32 = self.size as i32;
        if self.is_started() {
            let (x, y) = match dir {
                Direction::Down => (x, y + HV_OFFSET),
                Direction::DownRight => (x + DIAG_OFFSET, y + DIAG_OFFSET),
                Direction::Right => (x + HV_OFFSET, y),
                Direction::UpRight => (x + DIAG_OFFSET, y - DIAG_OFFSET),
                Direction::Up => (x, y - HV_OFFSET),
                Direction::UpLeft => (x - DIAG_OFFSET, y - DIAG_OFFSET),
                Direction::Left => (x - HV_OFFSET, y),
                Direction::DownLeft => (x - DIAG_OFFSET, y + DIAG_OFFSET),
            };
            if x>= 0 && y >= 0 && x < size && y < size && self.value_at(x as usize, y as usize) == 0 as u8 {
                return Some((x as usize, y as usize));
            }
        }
        None
    }

    /// Return true if the board is complete. A board is complete if the value
    /// of the last move equals the maximum number of cells, and there are no
    /// empty cells in the board.
    pub fn is_won(&self) -> bool {
        static ZERO: u8 = 0 as u8;
        self.value_at(self.x, self.y) == self.cells as u8 && !self.values.contains(&ZERO)
    }

    /// Return `true` if there are no possible moves for the current board.
    pub fn is_blocked(&self) -> bool {
        self.is_started() && self.possible_moves().len() == 0 
    }

    /// The score is simply the highest value on the board.
    pub fn score(&self) -> usize {
        self.values.iter().cloned().fold(0, u8::max) as usize
    }
    
    /// Return the value at the given location on the board.
    pub fn value_at(&self, x: usize, y: usize) -> u8 {
        self.values[y * self.size + x]
    }

    /// Start the puzzle by placing a 1 in the given location.
    pub fn start_at(&mut self, x: usize, y: usize) -> Result<Board, BoardError> {
        self.set_value(x, y, 1)
    }
    
    /// Make the next move on the board using a given direction.
    pub fn next_move(&mut self, dir: Direction) -> Result<Board, BoardError> {
        if !self.is_started() {
            return Err(BoardError::new("Attempt to move with an empty board"));
        }
        match self.valid_move(dir) {
            Some((x, y)) => self.set_value(x, y, self.value_at(self.x, self.y) + 1),
            None => Err(BoardError::new(&format!("Moving in direction: '{}' is invalid", dir))),
        }
    }

    /// Set the value of location on the board to `value`.
    fn set_value(&mut self, x: usize, y: usize, value: u8) -> Result<Board, BoardError> {
        if x >= self.size || y >= self.size {
            return Err(BoardError::new(&format!("cannot set cell [{}, {}], out of range ({})", x, y, self.size)));
        }
        if value < 1 {
            return Err(BoardError::new(&format!("cannot clear cell [{}, {}]", x, y)));
        }
        if value <= self.score() as u8 {
             return Err(BoardError::new(&format!("cannot set cell [{}, {}] = {}, value already used", x, y, value)));
        }
        if value > self.cells as u8 {
            return Err(BoardError::new(&format!("cannot set cell [{}, {}] = {}, larger than ({})", x, y, value, self.cells)));
        }
        if self.value_at(x, y) != 0 {
            return Err(BoardError::new(&format!("cannot change value of cell [{}, {}]", x, y)));
        }
        let mut board = self.clone();
        board.x = x;
        board.y = y;
        board.values[y * self.size + x] = value;
        Ok(board)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // Start a board and check that invariants hold.
    fn new_board() {
        let mut board = Board::new(10);
        // newly created board has a size of 10
        assert_eq!(board.size, 10);
        // newly created bboard has cell count of 100
        assert_eq!(board.cells, 100);
        // newly created board has a score of 0
        assert_eq!(board.score(), 0);
        // newly created board is not started
        assert_eq!(board.is_started(), false);
        // unstarted board cannot be won
        assert_eq!(board.is_won(), false);
        // no possible moves because board isn't started
        assert_eq!(board.possible_moves().len(), 0);
        // start the board
        board = board.start_at(5, 5).unwrap();
        // board is started
        assert_eq!(board.is_started(), true);
        // cell at (5, 5) should be 1
        assert_eq!(board.values[55], 1);
        // score is 1
        assert_eq!(board.score(), 1);
        // all moves should be possible
        assert_eq!(board.possible_moves().len(), 8);
        // board isn't won
        assert_eq!(board.is_won(), false);
    }

    #[test]
    // Test a complete set of moves on a 5x5 board and confirm that all
    // invariants hold. The completed board looks like:
    //
    //      1 24 14  2 25
    //     16 21  5  8 20
    //     13 10 18 23 11
    //      4  7 15  3  6
    //     17 22 12  9 19
    fn win_5() {
        let moves = [
            Direction::Right, Direction::Down, Direction::Left, 
            Direction::UpRight, Direction::DownRight, Direction::Left,
            Direction::UpRight, Direction::Down, Direction::UpLeft,
            Direction::Right, Direction::DownLeft, Direction::UpLeft,
            Direction::UpRight, Direction::Down, Direction::UpLeft,
            Direction::Down, Direction::UpRight, Direction::DownRight,
            Direction::Up, Direction::Left, Direction::Down,
            Direction::UpRight, Direction::UpLeft, Direction::Right
        ];
        // The number of possible moves on the board after each move.
        let possible_moves = [
            3, 2, 2, 1, 2, 
            2, 2, 2, 2, 1,
            2, 1, 2, 1, 2,
            1, 1, 2, 2, 1,
            1, 1, 1, 1, 0,
        ];

        let mut board = Board::new(5);
        assert_eq!(board.is_started(), false);
        board = board.start_at(0, 0).unwrap();
        assert_eq!(board.is_started(), true);
        let mut possible = possible_moves.iter();
        let mut i = 1;
        for m in moves.iter() {
            assert_eq!(board.possible_moves().len(), *possible.next().unwrap(), "testing move {}", i);
            assert_eq!(board.is_won(), false);
            assert_eq!(board.is_blocked(), false);
            assert_eq!(board.score(), i);
            let ret = board.next_move(*m);
            assert_eq!(ret.is_ok(), true, "testing move {}", i);
            board = ret.unwrap();
            i += 1;
        }
        assert_eq!(board.possible_moves().len(), *possible.next().unwrap());
        // ensure we have checked all values
        assert_eq!(possible.next().is_none(), true);
        // board is now won
        assert_eq!(board.is_won(), true);
        // score should be 25 (max board)
        assert_eq!(board.score(), board.cells);
        // there should be no possible moves;
        assert_eq!(board.is_blocked(), true);
    }
}