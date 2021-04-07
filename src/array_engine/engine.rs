use crate::engine_core::{
    piece_old::{
        Piece,
        p
    },
    square::Square,
    engine::Engine,
    helpers::{
        file_to_number,
        vec_to_arr,
        s,
    }
};

use super::{fen_helpers::parse_rank};


pub struct AEngine {
    board: [[Piece; 8]; 8],
}

impl Engine for AEngine {
    fn new() -> Self {
        Self {
            board: [
                [p("R"),p("N"),p("B"),p("Q"),p("K"),p("B"),p("N"),p("R")],
                [p("P"),p("P"),p("P"),p("P"),p("P"),p("P"),p("P"),p("P")],
                [p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" ")],
                [p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" ")],
                [p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" ")],
                [p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" "),p(" ")],
                [p("p"),p("p"),p("p"),p("p"),p("p"),p("p"),p("p"),p("p")],
                [p("r"),p("n"),p("b"),p("q"),p("k"),p("b"),p("n"),p("r")],
            ]
        }
    }

    fn board_state_from_fen(fen_str: &str) -> Self {
        let ranks: Vec<&str> = fen_str.split("/").collect();

        let mut board: Vec<[Piece;8]> = Vec::new();

        for rank in ranks {
            board.push(parse_rank(rank));
        }

        board.reverse();

        return Self {
            board: vec_to_arr(board)
        }
    }


    fn board_state_from_pgn(fen_str: &str) -> Self {
        todo!()
    }

    fn board_state_from_8x8_str(str: &str) -> Self {
        todo!()
    }

    fn board_state_to_fen(&self) -> String {
        for rank in self.board.iter() {
            
        }

        return s("");
    }

    fn board_state_to_pgn(&self) -> String {
        todo!()
    }

    fn board_state_to_8x8_str(&self) -> String {
        todo!()
    }

    fn print(&self) {
        for row in 0..8 {
            println!("       ---------------------------------");
            for rank in 0..8{
                let piece = &self.board[row][rank].utf_piece;
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
        // Since the board is upside down in memore we need to 
        // calculate the correct rank. No biggie
        let rank_index = square.rank - 1;
        let file_index = file_to_number(&square.file);
        let occupant = &self.board[rank_index][file_index];

        return occupant;
    }
}


#[cfg(test)]
mod test {
    use crate::engine_core::{
        piece_old::{Piece,p},
        square::Square,
    };
    use crate::test::{
        mocks::mock_board_states::get_test_board_state,
    };

    use super::{
        *,
        super::helpers::str_to_square,
        super::fen_helpers::{
            rank_to_fen
        }
    };

    #[test]
    fn str_to_square_should_return_square() {
        let str = "e4";
        let square = str_to_square(str);
        let expected = Square::new("e", 4);
        assert_eq!(square, expected);
    }

    #[test]
    fn test_should_retrun_white_king() {
        let board = AEngine::new();
        let occupant = board.get_square_occupant(str_to_square("e1"));
        let white_king = p("K");

        assert_eq!(occupant.notation, white_king.notation);
    }

    #[test]
    fn should_parse_fen_rank() {
        let rank_str = "r2P3r";
        let correct_rank: [Piece; 8] = [p("r"),p(" "),p(" "),p("P"),p(" "),p(" "),p(" "),p("r")];
        let parsed_rank = parse_rank(rank_str);
        assert_eq!(parsed_rank, correct_rank);
    }

    #[test]
    fn should_parse_fen_state() {
        let fen_state = "r1q1r1k1/1p1nbpp1/p1n1p1p1/3p2P1/3P1P1P/2P3B1/PP1N2B1/R2Q1RK1";

        let engine = AEngine::board_state_from_fen(fen_state);

        engine.print();

        assert_eq!(engine.board, get_test_board_state());
    }

    #[test]
    fn should_output_fen_rank() {
        let test_rank = &get_test_board_state()[7];
        let output = rank_to_fen(&test_rank);
        let result = "r1q1r1k1";

        assert_eq!(output, result)
    }
}
