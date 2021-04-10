
use core::panic;
use crate::engine_core::{
    piece::{
        Piece,
        p_to_notation
    },
};

pub fn rank_to_fen(rank: &[Piece; 8]) -> String {
   let mut space_count = 0;
   let mut result = String::new();
   
   for square in rank.iter() {
       match square {
           Piece::Empty => space_count += 1,
           Piece::Error => panic!("Yikes!"),
           _ => {
               result.push_str(&fenify(space_count, p_to_notation(square)));
               space_count = 0;
           }
       }
   }

   if space_count > 0 {
        result.push_str(&space_count.to_string());
   }

   return result;
}

pub fn board_to_fen(board: &[[Piece; 8];8]) -> String { 
    let mut result_vec: Vec<String> = Vec::new();
    for rank in board {
        result_vec.push(rank_to_fen(rank));
    }
    result_vec.reverse();
    return result_vec.join("/");
}

fn fenify(spaces: i32, piece: String) -> String {
    if spaces > 0 {
        return format!("{}{}", spaces, piece);
    } else {
        return piece;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        test::mocks::mock_board_states::{
            get_test_board_state,
            get_test_fen
        }
    };

    #[test]
    fn should_output_fen_rank() {
        let test_rank = &get_test_board_state()[7];
        let output = rank_to_fen(&test_rank);
        let result = "r1q1r1k1";

        assert_eq!(output, result)
    }

    #[test]
    fn should_output_board_as_fen() {
        let correct_fen = get_test_fen();
        let board = board_to_fen(&get_test_board_state());

        assert_eq!(board, correct_fen);
    }
}