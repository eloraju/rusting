use super::{board::{Engine, Square, Occupant}, board_utils::file_to_number};
use super::board_utils::{char_to_piece,s};
use regex::Regex;

pub struct ABoard {
    board: [[String; 8]; 8],
}

impl Engine for ABoard {
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
        let ranks: Vec<&str> = fen_str.split("/");
        let board = [[" ".to_string(); 8]; 8];


        for (r_index, file) in ranks.iter().enumerate() {
        }

        return Self {
            board
        }

    }

    fn parse_rank(rank: &str) -> [String;8] {
        lazy_static! {
            static ref PIECE_RE: Regex = Regex::new(r"[rnbqkpRNBQKP]").unwrap();
        }

        let rank_arr = rank.chars().collect::<Vec<char>>();
        let mut result = [String: 8];
        let mut space_counter = 0;
        for index in 0..8 {
            if space_counter > 0 {
                space_counter -= 1;
                result[index] = " ".to_string();
                continue;
            }
        }

        return result;
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
        let rank_index = square.rank - 1;
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

    #[test]
    fn should_parse_rank_correctly() {
        let rank_str = "r2P3r";
        let correct_rank: [String; 8] = [s("r"),s(" "),s(" "),s("P"),s(" "),s(" "),s(" "),s("r")];
        let parsed_rank = parse_rank(rank_str);
        assert_eq!(parsed_rank, correct_rank);
    }
}
