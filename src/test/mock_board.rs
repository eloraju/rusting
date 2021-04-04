use crate::boards::board::{Engine, Occupant, Square};


pub struct MockBoard {
    board: String,
}

impl Engine for MockBoard {
    fn new() -> Self {
        Self {
            board: "Mock".to_string()
        }
    }

    fn from_fen(fen_str: &str) -> Self {
        todo!()
    }

    fn from_pgn(fen_str: &str) -> Self {
        todo!()
    }

    fn from_8x8_str(str: &str) -> Self {
        todo!()
    }

    fn to_fen() -> String {
        todo!()
    }

    fn to_pgn() -> String {
        todo!()
    }

    fn to_8x8_str() -> String {
        todo!()
    }

    fn get(&self, square: Square) -> Occupant {
        Occupant::Piece("K".to_string())
    }

    fn print(&self) {
        todo!()
    }
}
