
use regex::Regex;
use crate::engine_core::{
    piece::{Piece, p},
    helpers::vec_to_arr
};

pub fn parse_rank(rank: &str) -> [Piece;8] {
    lazy_static! {
        static ref PIECE_RE: Regex = Regex::new(r"[1-8]").unwrap();
    }

    let mut collector: Vec<Piece> = Vec::new();

    for char in rank.chars() {
        // Is there a better way to do this?
        let as_str: &str = &char.to_string();
        if PIECE_RE.is_match(as_str) {
            let empty_count: usize = char.to_digit(10).unwrap() as usize;
            for _ in 0..empty_count {
                collector.push(p(" "));
            }
            continue;
        }

        collector.push(p(&char.to_string()));
    }

    let result: [Piece; 8] = vec_to_arr(collector);
    return result;
}

pub fn fen_to_board(fen_str: &str) -> [[Piece; 8];8] {
    let ranks: Vec<&str> = fen_str.split("/").collect();
    let mut board: Vec<[Piece;8]> = Vec::new();
    for rank in ranks {
        board.push(parse_rank(rank));
    }

    board.reverse();

    return vec_to_arr(board);
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        test::mocks::mock_board_states::{
            get_test_board_state,
            get_test_fen
        },
    };

    #[test]
    fn should_parse_fen_rank() {
        let rank_str = "r2P3r";
        let correct_rank: [Piece; 8] = [p("r"),p(" "),p(" "),p("P"),p(" "),p(" "),p(" "),p("r")];
        let parsed_rank = parse_rank(rank_str);
        assert_eq!(parsed_rank, correct_rank);
    }

    #[test]
    fn should_parse_fen_to_board() {
        let board_str = get_test_fen();
        let parsed_board = fen_to_board(&board_str);
        assert_eq!(parsed_board, get_test_board_state());
    }
}