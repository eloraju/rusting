use std::convert::TryInto;
use regex::Regex;

use crate::engine_core::{
    piece::Piece,
    square::Square
};


pub fn vec_rank_to_arr(v: Vec<Piece>) -> [Piece; 8] {
    match v.try_into() {
        Ok(array) => array,
        Err(uhoh) => panic!("Exptected len {} but got {}", 8, uhoh.len())
    }
}

pub fn str_to_square(square: &str) -> Square {
    lazy_static! {
        static ref SQUARE_RE: Regex = Regex::new(r"^(?P<file>[a-h])(?P<rank>[1-8])$").unwrap();
    }

    let caps = SQUARE_RE.captures(square).unwrap();

    // convert tile to char
    // This ugly
    let file = &caps["file"];
    // convert rank to u8
    let rank = caps["rank"].parse::<usize>().unwrap();

    return Square::new(file, rank)
}