use super::{
    square::Square,
    piece_old::Piece
};

pub trait Engine {
    fn new() -> Self;

    // Input board state
    fn board_state_from_fen(fen_str: &str) -> Self;
    fn board_state_from_pgn(fen_str: &str) -> Self;
    fn board_state_from_8x8_str(str: &str) -> Self;

    // Output board state 
    fn board_state_to_fen(&self) -> String;
    fn board_state_to_pgn(&self) -> String;
    fn board_state_to_8x8_str(&self) -> String;

    // Something something something
    fn get_square_occupant(&self, square: Square) -> &Piece;

    // Move generation
    // Move valdiation
    // Rule enfrocement
    // Position evalution

    fn print(&self);
}
