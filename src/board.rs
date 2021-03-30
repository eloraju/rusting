//trait InitableFromState<T>{
//    fn init_with_state(&self, state: T) -> Self;
//}
//
// Board can be initialized with portable game notation
// This means that I'll have to write a PGN parser... oh boiiii
//trait InitableFromPGN {
//    fn init_with_pgn(&self, pgn: &str) -> Self;
//}

// remove this on a later date
#![allow(dead_code)]

struct Game {
    w_can_castle: bool,
    w_en_passant: bool,
    b_can_castle: bool,
    b_en_passant: bool,
    check: bool,
    turn_count: f32, // interget value = white, fractional value = black
}

impl Game {
    fn new() -> Self {
        Self {
            w_can_castle: true,
            w_en_passant: true,
            b_can_castle: true,
            b_en_passant: true,
            check: false,
            turn_count: 0.0
        }
    }

}

pub struct ArrayBoard {
    // 8*8 string array to keep track of pieces on the board
    // Quite possible the most naive approach one can take, but
    // I don't want to optimize just now. I just want to get something
    // working :)
    rules: Game,
    array_board: [[String; 8]; 8],
    // This should make it trivial to move backwards and forwards in the 
    // move history
    history: Vec<ArrayBoard>,
}

fn number_to_char(num: u32) -> char {
    match num {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => '!',
    }
}

fn char_to_number(char: char) -> u32 {
    match char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => -1,
    }
}

//  More sutff into the backlog :)
//impl InitableFromState<&str> for ArrayBoard {
//    fn init_with_state(&self, state: &str) -> Self {
//
//    }
//}

impl ArrayBoard {

    /*
     * Create new board that should look something like this
     *   ---------------------------------
     * 8 | r | n | b | q | k | b | n | r |
     *   |-------------------------------|
     * 7 | p | p | p | p | p | p | p | p |
     *   |-------------------------------|
     * 6 |   |   |   |   |   |   |   |   |
     *   |-------------------------------|
     * 5 |   |   |   |   |   |   |   |   |
     *   |-------------------------------|
     * 4 |   |   |   |   |   |   |   |   |
     *   |-------------------------------|
     * 3 |   |   |   |   |   |   |   |   |
     *   |-------------------------------|
     * 2 | P | P | P | P | P | P | P | P |
     *   |-------------------------------|
     * 1 | R | N | B | Q | K | B | N | R |
     *   ---------------------------------
     *     a   b   c   d   e   f   g   h
     *
     */


    pub fn new() -> Self {
        Self {
            rules: Game::new(),
            history: Vec::new(),
            array_board: [
                [String::from("r"),String::from("n"),String::from("b"),String::from("q"),String::from("k"),String::from("b"),String::from("n"),String::from("r")],
                [String::from("p"),String::from("p"),String::from("p"),String::from("p"),String::from("p"),String::from("p"),String::from("p"),String::from("p")],
                [String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" ")],
                [String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" ")],
                [String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" ")],
                [String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" "),String::from(" ")],
                [String::from("P"),String::from("P"),String::from("P"),String::from("P"),String::from("P"),String::from("P"),String::from("P"),String::from("P")],
                [String::from("R"),String::from("N"),String::from("B"),String::from("Q"),String::from("K"),String::from("B"),String::from("N"),String::from("R")],
            ]
        }
    }

    pub fn print(&self) {
        for row in 0..8 {
            println!("  ---------------------------------");
            for rank in 0..8{
                let occupant = &self.array_board[row][rank];
                match rank {
                    0 => print!("{} | {} | ", 8 - row, occupant),
                    7 => {
                        println!("{} |", occupant);
                    },
                    _ => print!("{} | ", occupant),
                }

            };
        };
        println!("  ---------------------------------");
        println!("    a   b   c   d   e   f   g   h");
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
