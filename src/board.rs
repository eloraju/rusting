use String::from as str;
use rusting::lib::char_to_piece;
pub mod util;
// remove this on a later date
#![allow(dead_code)]

pub struct ABoardHistory {
}

pub struct ABoard {
    state: [[String; 8]; 8],
}


//  More sutff into the backlog :)
//impl InitableFromState<&str> for ArrayBoard {
//    fn init_with_state(&self, state: &str) -> Self {
//
//    }
//}

impl ABoard {
    pub fn new() -> Self {
        Self {
            state: [
                [str("R"),str("N"),str("B"),str("Q"),str("K"),str("B"),str("N"),str("R")],
                [str("P"),str("P"),str("P"),str("P"),str("P"),str("P"),str("P"),str("P")],
                [str(" "),str(" "),str(" "),str(" "),str(" "),str(" "),str(" "),str(" ")],
                [str(" "),str(" "),str(" "),str(" "),str(" "),str(" "),str(" "),str(" ")],
                [str(" "),str(" "),str(" "),str(" "),str(" "),str(" "),str(" "),str(" ")],
                [str(" "),str(" "),str(" "),str(" "),str(" "),str(" "),str(" "),str(" ")],
                [str("p"),str("p"),str("p"),str("p"),str("p"),str("p"),str("p"),str("p")],
                [str("r"),str("n"),str("b"),str("q"),str("k"),str("b"),str("n"),str("r")],
            ]
        }
    }

    pub fn print(&self) {
        println!();
        println!(" --- Welcome to the nameless chess engine! ---");
        println!();
        for row in 0..8 {
            println!("       ---------------------------------");
            for rank in 0..8{
                let piece = char_to_piece(&self.state[row][rank]);
                match rank {
                    0 => print!("     {} | {} | ", 8 - row, piece),
                    7 => {
                        println!("{} |", piece);
                    },
                    _ => print!("{} | ", piece),
                }

            };
        };
        println!("       ---------------------------------");
        println!("         a   b   c   d   e   f   g   h");
    }
}

