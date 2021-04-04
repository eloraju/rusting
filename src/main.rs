mod engines;
mod test;
use engines::array_engine::ABoard;
use engines::engine::Engine;

#[macro_use]
extern crate lazy_static;

mod game;

fn main() {
    let board  = ABoard::new();
    board.print();
}
