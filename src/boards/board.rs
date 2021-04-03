use std::fmt::Debug;

#[derive(Debug)]
pub struct Square {
    pub file: char,
    pub rank: usize,
}

impl Square {
    pub fn new(file:char, rank: usize) -> Self {
        Self {
            file,
            rank
        }
    }
}

impl PartialEq for Square {
    fn eq(&self, other: &Self) -> bool {
        return self.file == other.file && self.rank == other.rank;
    }
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum Occupant {
    Empty,
    Piece(String)
}

pub trait Board {
    fn new() -> Self;
    fn from_fen(fen_str: &str) -> Self;
    fn from_pgn(fen_str: &str) -> Self;
    fn from_8x8_str(str: &str) -> Self;
    fn to_fen() -> String;
    fn to_pgn() -> String;
    fn to_8x8_str() -> String;
    fn get(&self, square: Square) -> Occupant;
    fn print(&self);
}
