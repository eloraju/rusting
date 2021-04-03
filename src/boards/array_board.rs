use super::board::Board;
use super::board_utils::{char_to_piece,s};

pub struct ABoard {
    state: [[String; 8]; 8],
}

impl ABoard {
    pub fn new() -> Self {
        Self {
            state: [
                [s("R"),s("N"),s("B"),s("Q"),s("K"),s("B"),s("N"),s("R")],
                [s("P"),s("P"),s("P"),s("P"),s("P"),s("P"),s("P"),s("P")],
                [s(" "),s(" "),s(" "),s(" "),s(" "),s(" "),s(" "),s(" ")],
                [s(" "),s(" "),s(" "),s(" "),s(" "),s(" "),s(" "),s(" ")],
                [s(" "),s(" "),s(" "),s(" "),s(" "),s(" "),s(" "),s(" ")],
                [s(" "),s(" "),s(" "),s(" "),s(" "),s(" "),s(" "),s(" ")],
                [s("p"),s("p"),s("p"),s("p"),s("p"),s("p"),s("p"),s("p")],
                [s("r"),s("n"),s("b"),s("q"),s("k"),s("b"),s("n"),s("r")],
            ]
        }
    }

}

impl Board for ABoard {
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

    fn print(&self) {
        for row in 0..8 {
            println!("       ---------------------------------");
            for rank in 0..8{
                let piece = char_to_piece(&self.state[row][rank]);
                match rank {
                    0 => print!("     {} | {} | ", 8 - row, piece),
                    7 => println!("{} |", piece),
                    _ => print!("{} | ", piece),
                }
            };
        };
        println!("       ---------------------------------");
        println!("         a   b   c   d   e   f   g   h");
    }
}
