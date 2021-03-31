enum Turn {
    WHITE,
    BLACK
}

struct Square {
}

struct History<T: Board> {
    state: T,
    prev_state: &'History<T>,
    next_state: &'History<T>,
}

struct Move {
    piece: String,
    row: u8,
    rank: String,
}

trait Board {
    fn as_string(&self) -> String;
    fn print(&self);
    fn play_move(&self, piece: Piece, square: Square) -> Self;
}

// Board history should implement a function that returns the last board state
struct Game<T: Board> {
    en_passant: Option<Square>,
    w_k_castle: bool,
    w_q_castle: bool,
    b_k_castle: bool,
    b_q_castle: bool,
    turn_count: u32,
    turn: Turn,
    board: T,
    prev_turn: Option<&History<T>>,
    next_turn: Option<&History<T>>
}


impl Game{
    fn new(board: Board) -> Self {
        Self {
            en_passant: None,
            w_k_castle: true,
            w_q_castle: true,
            b_k_castle: true,
            b_q_castle: true,
            turn_count: 0,
            turn: Turn::WHITE,
            prev_turn: None,
            next_turn: None,
            board,
        }
    }
}
