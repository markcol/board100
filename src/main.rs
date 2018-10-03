/*!

A Rust solver for a simple numerical game.

See the inital article at [simple-number].

# Rules

The rules are simple: on an empty 10x10 grid (100 squares in total) you put a
number 1 on an arbitrary square. Starting from that square you can move
horizontally or vertically jumping over two squares or diagonally jumping over
one square. There you can place number 2. Your task is to reach number 100,
filling all squares. You can not visit already visited squares.

Here is an example of a solved game with a reduced 5x5 grid, starting at
top-left corner:

     1 24 14  2 25
    16 21  5  8 20
    13 10 18 23 11
     4  7 15  3  6
    17 22 12  9 19



[simple-number]: https://www.nurkiewicz.com/2018/09/brute-forcing-seemingly-simple-number.html
 */

use std::slice::Iter;

const HV_OFFSET: usize = 3;
const DIAG_OFFSET: usize = 2;

#[derive(Debug)]
enum Direction {
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

struct Board {
    size: usize,
    cells: usize,
    values: Vec<u8>,
    x: usize, // last move location
    y: usize,
}

impl Board {
    pub fn new(size: usize) -> Self {
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

    /// Return true if the board has been started.
    pub fn is_started(self) -> bool {
        self.values[self.y * self.size + self.x] > 0
    }

    /// Return a list of all possible moves from the current location.
    /// Returns an empty list if there are no moves, or the board is empty.
    pub fn possible_moves(self) -> Vec<Direction> {
        let mut moves: Vec<Direction> = Vec::new();
        let val = self.values[self.y * self.size + self.x];
        if val > 0 {
            for dir in Direction::iterator() {
                match self.valid_move(*dir) {
                    Some(_) => moves.push(*dir),
                    None => {}
                }
            }
        }
        moves
    }

    /// Determines if a move in the given direction is valid. If the move is
    /// valid, it returns Some((x, y)) where (x, y) is the cell location
    /// resulting from the move. Otherwise, returns None.
    fn valid_move(self, dir: Direction) -> Option<(u8, u8)> {
        let (x, y) = match dir {
            Direction::Down => (self.x, self.y + HV_OFFSET),
            Direction::DownRight => (self.x + DIAG_OFFSET, self.y + HV_OFFSET),
            Direction::Right => (self.x + HV_OFFSET, self.y),
            Direction::UpRight => (self.x + DIAG_OFFSET, self.y - DIAG_OFFSET),
            Direction::Up => (self.x, self.y - HV_OFFSET),
            Direction::UpLeft => (self.x - DIAG_OFFSET, self.y - DIAG_OFFSET),
            Direction::Left => (self.x - HV_OFFSET, self.y),
            Direction::DownLeft => (self.x - DIAG_OFFSET, self.y + DIAG_OFFSET),
        };
        if x >= 0 && y >= 0 && x < self.size && y < self.size {
            return Some((x as u8, y as u8));
        }
        None
    }

    /// Make the next move on the board using a given direction.
    pub fn next_move(&mut self, dir: Direction) -> Result<(), Err> {
        if !self.is_started() {
            return Err("Attempt to move with an empty board");
        }
        let val = self.values[self.y * self.size + self.x];
        match self.valid_move(dir) {
            Some((x, y)) => self.set_cell(x, y, val + 1),
            None => Err(format!("Moving in direction: {} is invalid", dir)),
        }
    }

    /// Return true if the board is complete. A board is complete if the value
    /// of the last move equals the maximum number of cells, and there are no
    /// empty cells in the board.
    pub fn is_won(self) -> bool {
        self.values[self.y * self.size + self.x] == self.cells as u8 && !self.values.contains(0)
    }

    /// Start the puzzle by placing a 1 in the given location.
    pub fn start_at(&mut self, x: usize, y: usize) -> Result<(), Err> {
        self.set_value(x, y, 1)
    }

    /// The score is simply the highest value on the board.
    pub fn score(self) -> usize {
        self.values.iter().cloned().fold(0 / 0, u8::max) as usize
    }

    /// Set the value of location on the board to `value`.
    fn set_value(&mut self, x: usize, y: usize, value: u8) -> Result<(), Err> {
        if value < 1 {
            return Err(format!("cannot clear cell [{}, {}]", x, y));
        }
        if value as usize > self.cells {
            return Err(format!(
                "cannot set cell [{}, {}] to {} (max: {})",
                x, y, value, self.cells
            ));
        }
        self.set_cell(x, y, value)
    }

    // TODO(markcol): use Index, IndexRef traits instead?
    fn set_cell(&mut self, x: usize, y: usize, value: u8) -> Result<(), Err> {
        if x >= self.size || y >= self.size {
            return Err(format!(
                "index [{}, {}] out of range (max: {})",
                x, y, self.size
            ));
        }
        if self.values[y * self.size + x] != 0 {
            return Err(format!("cannot change value of cell [{}, {}]", x, y));
        }
        self.x = x;
        self.y = y;
        self.values[y * self.size + x] = value;
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
