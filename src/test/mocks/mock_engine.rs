use crate::engine_core::{
    engine::Engine,
    piece::{
        Piece,
        p
    },
    square::Square,
};

pub struct MockEngine {
    _board: String,
    white_king: Piece
}
impl Engine<String> for MockEngine {
    fn new() -> Self {
        Self {
            _board: "Mock".to_string(),
            white_king: p("K")
        }
    }

    fn board(&self) -> &String {
        &self._board
    }

    fn state_from_fen(_fen_str: &str) -> Self {
        todo!()
    }

    fn state_from_pgn(_fen_str: &str) -> Self {
        todo!()
    }

    fn state_to_fen(&self) -> String {
        todo!()
    }

    fn state_to_pgn(&self) -> String {
        todo!()
    }

    fn get_square_occupant(&self, _square: Square) -> &Piece {
        &self.white_king
    }

    fn print(&self) {
        todo!()
    }
}
