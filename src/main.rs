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

#![feature(custom_attribute)]

mod board;

fn main() {
    println!("Hello, world!");
}
