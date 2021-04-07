use crate::engine_core::{
    engine::Engine,
    piece_old::{
        Piece,
        p
    },
    square::Square,
};


pub struct MockEngine {
    board: String,
    white_king: Piece
}

impl Engine for MockEngine {
    fn new() -> Self {
        Self {
            board: "Mock".to_string(),
            white_king: p("K")
        }
    }

    fn board_state_from_fen(fen_str: &str) -> Self {
        todo!()
    }

    fn board_state_from_pgn(fen_str: &str) -> Self {
        todo!()
    }

    fn board_state_from_8x8_str(str: &str) -> Self {
        todo!()
    }

    fn board_state_to_fen(&self) -> String {
        todo!()
    }

    fn board_state_to_pgn(&self) -> String {
        todo!()
    }

    fn board_state_to_8x8_str(&self) -> String {
        todo!()
    }

    fn get_square_occupant(&self, square: Square) -> &Piece {
        &self.white_king
    }

    fn print(&self) {
        todo!()
    }
}
