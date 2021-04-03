use crate::Board;
use crate::boards::board::Square;
pub enum Turn {
    WHITE,
    BLACK
}


pub struct State<T: Board> {
    en_passant: Option<Square>,
    w_k_castle: bool,
    w_q_castle: bool,
    b_k_castle: bool,
    b_q_castle: bool,
    turn_count: u16,
    turn: Turn,
    board: T
}


impl<T> State<T> 
where T: Board {
    fn new(board: T) -> Self {
        Self {
            en_passant: None,
            w_k_castle: true,
            w_q_castle: true,
            b_k_castle: true,
            b_q_castle: true,
            turn_count: 0,
            turn: Turn::WHITE,
            board,
        }
    }
}
