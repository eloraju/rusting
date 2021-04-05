use regex::Regex;
use crate::engine_core::{
    piece::{Piece, p},
};
use super::helpers::vec_rank_to_arr;

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

    let result: [Piece; 8] = vec_rank_to_arr(collector);
    return result;
}

// This function should not consume the values it is given
pub fn rank_to_fen(rank: &[Piece; 8]) -> String {
   let mut space_count = 0;
   let mut result = String::new();
   
   for square in rank.iter() {
   }

   return result;
}