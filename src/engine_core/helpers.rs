use std::convert::TryInto;

/// Transforms a alphabetic file identifier into a number
pub fn file_to_number(char: &str) -> usize {
    match char {
        "a" => 0,
        "b" => 1,
        "c" => 2,
        "d" => 3,
        "e" => 4,
        "f" => 5,
        "g" => 6,
        "h" => 7,
        _ => 99,
    }
}

/// Transforms given character into its UTF-8 representation of a chess piece
pub fn char_to_piece(input: &str)-> &'static str {
    match input {
        "K"=> &"♔",
        "Q"=> &"♕",
        "R"=> &"♖",
        "B"=> &"♗",
        "N"=> &"♘",
        "P"=> &"♙",
        "k"=> &"♚",
        "q"=> &"♛",
        "r"=> &"♜",
        "b"=> &"♝",
        "n"=> &"♞",
        "p"=> &"♟︎",
        " "=> &" ",
        _  => &"💩",
    }
}

/// Transforms a vector into an array
pub fn vec_to_arr<T, const N:usize>(vector: Vec<T>) -> [T; N] {
    match vector.try_into() {
        Ok(array) => array,
        Err(uhoh) => panic!("Exptected len {} but got {}", N, uhoh.len())
    }
}
