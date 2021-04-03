mod boards;
mod test;
use boards::array_board::ABoard;
use boards::board::Board;

#[macro_use]
extern crate lazy_static;

mod game;

fn main() {
    let board  = ABoard::new();
    board.print();
}
