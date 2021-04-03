mod boards;
use boards::array_board::ABoard;
use boards::board::Board;

mod game;

fn main() {
    let board  = ABoard::new();
    board.print();
}
