use crate::Engine;
use crate::boards::board::Square;
pub enum Turn {
    WHITE,
    BLACK
}


pub struct State<T: Engine> {
    pub en_passant: Option<Square>,
    pub w_k_castle: bool,
    pub w_q_castle: bool,
    pub b_k_castle: bool,
    pub b_q_castle: bool,
    pub turn_count: u16,
    pub turn: Turn,
    pub board: T
}


impl<T> State<T> 
where T: Engine {
    pub fn new(board: T) -> Self {
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
