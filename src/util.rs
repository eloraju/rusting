use String::from as str;


pub fn number_to_char(num: u32) -> char {
    match num {
        0 => 'a',
        1 => 'b',
        2 => 'c',
        3 => 'd',
        4 => 'e',
        5 => 'f',
        6 => 'g',
        7 => 'h',
        _ => '!',
    }
}

pub fn char_to_number(char: char) -> u32 {
    match char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 99,
    }
}

pub fn char_to_piece(input: &str)-> String {
    match input {
        "K"=>str("♔"),
        "Q"=> str("♕"),
        "R"=> str("♖"),
        "B"=> str("♗"),
        "N"=> str("♘"),
        "P"=> str("♙"),
        "k"=> str("♚"),
        "q"=> str("♛"),
        "r"=> str("♜"),
        "b"=> str("♝"),
        "n"=> str("♞"),
        "p"=> str("♟︎"),
        " "=> str(" "),
        _  => str("💩"),
    }
}
