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
