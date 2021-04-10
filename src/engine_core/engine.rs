use super::{
    square::Square,
    piece::Piece
};

pub trait Engine<BoardType> {
    fn new() -> Self;

    fn board(&self) -> &BoardType;
    // Input state
    fn state_from_fen(fen_str: &str) -> Self;
    fn state_from_pgn(fen_str: &str) -> Self;

    // Output state 
    fn state_to_fen(&self) -> String;
    fn state_to_pgn(&self) -> String;

    // Something something something
    fn get_square_occupant(&self, square: Square) -> &Piece;

    // Move generation
    // Move valdiation
    // Rule enfrocement
    // Position evalution

    fn print(&self);
}
