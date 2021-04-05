use std::convert::TryInto;


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

pub fn char_to_piece(input: &str)-> String {
    match input {
        "K"=> s("♔"),
        "Q"=> s("♕"),
        "R"=> s("♖"),
        "B"=> s("♗"),
        "N"=> s("♘"),
        "P"=> s("♙"),
        "k"=> s("♚"),
        "q"=> s("♛"),
        "r"=> s("♜"),
        "b"=> s("♝"),
        "n"=> s("♞"),
        "p"=> s("♟︎"),
        " "=> s(" "),
        _  => s("💩"),
    }
}

pub fn s(s:&str) -> String {
    s.to_string()
}

pub fn vec_to_arr<T, const N:usize>(vector: Vec<T>) -> [T; N] {
    match vector.try_into() {
        Ok(array) => array,
        Err(uhoh) => panic!("Exptected len {} but got {}", N, uhoh.len())
    }
}