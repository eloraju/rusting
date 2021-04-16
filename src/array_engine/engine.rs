use crate::engine_core::{engine::Engine, helpers::{
        file_to_number,
    },
    history::History,
    state::State,
    square::Square,
    piece::{
        Piece,
        p,
        p_to_utf,
    }, 
};

use super::{fen::{
    input::fen_to_board,
    output::board_to_fen
}};


pub struct AEngine {
    history: History<[[Piece; 8]; 8]>
}

impl Engine<[[Piece; 8]; 8]> for AEngine {
    fn new() -> Self {
        Self {
            history: History::new(State::new([
                [p("R"),p("N"),p("B"),p("Q"),p("K"),p("B"),p("N"),p("R")],
                [p("P"),p("P"),p("P"),p("P"),p("P"),p("P"),p("P"),p("P")],
                [p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" ")],
                [p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" ")],
                [p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" ")],
                [p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" ")],
                [p("p"),p("p"),p("p"),p("p"),p("p"),p("p"),p("p"),p("p")],
                [p("r"),p("n"),p("b"),p("q"),p("k"),p("b"),p("n"),p("r")],
            ]))
        }
    }

    fn board(&self) -> &[[Piece; 8]; 8] {
        &self.history.state().board
    }

    fn state_from_fen(fen_str: &str) -> Self {
        return Self {
            history: History::new(State::new(fen_to_board(fen_str)))
        }
    }


    fn state_from_pgn(pgn_str: &str) -> Self {
        todo!()
    }

    fn state_to_fen(&self) -> String {
        // TODO: Convert rest of the state into fed. This is just the board
        return board_to_fen(&self.board());
    }

    fn state_to_pgn(&self) -> String {
        todo!()
    }

    fn print(&self) {
        for row in 0..8 {
            println!("       ---------------------------------");
            for rank in 0..8{
                let piece = p_to_utf(&self.board()[row][rank]);
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

    fn get_square_occupant(&self, square: Square) -> &Piece {
        let rank_index = square.rank - 1;
        let file_index = file_to_number(&square.file);
        let occupant = &self.board()[rank_index][file_index];

        return occupant;
    }
}


#[cfg(test)]
mod test {
    use crate::engine_core::{
        piece::{p, p_to_notation},
        engine::Engine
    };

    use super::{
        AEngine,
        super:: {
            helpers::{str_to_square},
        }
    };

    #[test]
    fn test_should_retrun_white_king() {
        let board = AEngine::new();
        let occupant = board.get_square_occupant(str_to_square("e1"));
        let white_king = p("K");

        assert_eq!(p_to_notation(&occupant), p_to_notation(&white_king));
    }
}
