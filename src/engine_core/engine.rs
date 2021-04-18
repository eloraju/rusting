use super::{
    square::Square,
    piece::Piece
};

pub trait Engine<BoardType> {
    /// Constructor
    fn new() -> Self;

    /// Getter for the board representation
    fn board(&self) -> &BoardType;
    // Input state

    /// Generate current state from given FEN string
    fn state_from_fen(fen_str: &str) -> Self;

    /// Generate state history from given PGN string
    fn state_from_pgn(fen_str: &str) -> Self;

    // Output state 

    /// Return the current state of the game as a FEN string
    fn state_to_fen(&self) -> String;
    /// Return the current state of the game as a PGN string
    fn state_to_pgn(&self) -> String;

    /// (Do I need this?) Return the Piece on a given square
    /// Should this return Option<&Piece>
    fn get_square_occupant(&self, square: Square) -> &Piece;

    // Move generation
    // Move valdiation
    // Rule enfrocement
    // Position evalution

    /// Print the current engine state into std_out
    fn print(&self);
}
