use core::panic;

use regex::Regex;
use crate::engine_core::{
    piece::{Piece, p, p_to_notation},
    helpers::{vec_to_arr, s}
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

// This function should not consume the values it is given
pub fn rank_to_fen(rank: &[Piece; 8]) -> String {
   let mut space_count = 0;
   let mut result = String::new();
   
   for square in rank.iter() {
       match square {
           Piece::Empty => space_count += 1,
           Piece::Error => panic!("Yikes!"),
           _ => {
               result.push_str(&fenify(space_count, &p_to_notation(square)))
           }
       }
   }

   return result;
}

fn fenify(spaces: i32, piece: &str) -> String {
    if spaces > 0 {
        let space_str = spaces.to_string();
        return space_str;
    } else {
        return piece.to_string();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{test::mocks::mock_board_states::get_test_board_state};

    #[test]
    fn should_parse_fen_rank() {
        let rank_str = "r2P3r";
        let correct_rank: [Piece; 8] = [p("r"),p(" "),p(" "),p("P"),p(" "),p(" "),p(" "),p("r")];
        let parsed_rank = parse_rank(rank_str);
        assert_eq!(parsed_rank, correct_rank);
    }

    #[test]
    fn should_output_fen_rank() {
        let test_rank = &get_test_board_state()[7];
        let output = rank_to_fen(&test_rank);
        let result = "r1q1r1k1";

        assert_eq!(output, result)
    }
}