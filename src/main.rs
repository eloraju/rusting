pub mod board;
pub mod game;

fn main() {
    let board  = ABoard::new();
    board.print();

    let game = Game::new(board);
}
