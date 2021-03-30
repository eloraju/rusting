trait InitableFromState<T>{
    fn init_with_state(&self, state: T) -> Self;
}

// Board can be initialized with portable game notation
// This means that I'll have to write a PGN parser... oh boiiii
//trait InitableFromPGN {
//    fn init_with_pgn(&self, pgn: &str) -> Self;
//}


struct Game {
    // 'can castle' basically means that 'has the king moved' 
    // there are other rules to castling as well, but those
    // should be handled when generating moves
    // or should they....
    w_can_castle: bool,
    w_en_passant: bool,
    b_can_castle: bool,
    b_en_passant: bool,
    check: bool,
    turn: bool, // true = white, false = black
    turn_count: u16, // better be safe than sorry
}

pub struct ArrayBoard {
    // 8*8 string array to keep track of pieces on the board
    // Quite possible the most naive approach one can take, but
    // I don't want to optimize just now. I just want to get something
    // working :)
    rules: Game,
    array_board: [[String; 8]; 8],
    history: Vec<ArrayBoard>,
}

impl ArrayBoard {

}

// 
impl InitableFromState<&str> for ArrayBoard {
    fn init_with_state(&self, state: &str) -> Self {

    }
}

// Bitboards... These seem extremly cool but at the same time
// a bit too much to handle. I'll definetly want to return to these!
//pub struct Bitboard {
//    // Bitboards
//    // White
//    w_k: u64,
//    w_q: u64,
//    w_r: u64,
//    w_b: u64,
//    w_n: u64,
//    w_p: u64,
//
//    // Black
//    b_k: u64,
//    b_q: u64,
//    b_r: u64,
//    b_b: u64,
//    b_n: u64,
//    b_p: u64,
//}
