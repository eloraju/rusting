use super::{board::{Board, Square, Occupant}, board_utils::file_to_number};
use super::board_utils::{char_to_piece,s};

pub struct ABoard {
    board: [[String; 8]; 8],
}

impl Board for ABoard {
    fn new() -> Self {
        Self {
            board: [
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
                let piece = char_to_piece(&self.board[row][rank]);
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

    fn get(&self, square: Square) -> Occupant {
        // Since the board is upside down in memore we need to 
        // calculate the correct rank. No biggie
        let rank_index = 9 - square.rank;
        let file_index = file_to_number(square.file);
        let occupant_str = &self.board[rank_index][file_index];

        if occupant_str == " " {
            return Occupant::Empty;
        }

        return Occupant::Piece(occupant_str.to_string());
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::board_utils::str_to_square;

    #[test]
    fn test_should_retrun_white_king() {
        let board = ABoard::new();
        let occupant = board.get(str_to_square("e1"));
        let white_king = Occupant::Piece(s("K"));

        assert_eq!(occupant, white_king);
    }
}