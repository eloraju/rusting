use super::board::Square;
use regex::Regex;


pub fn number_to_file(num: u32) -> char {
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

pub fn file_to_number(char: char) -> usize {
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

pub fn str_to_square(square: &str) -> Square {
    lazy_static! {
        static ref SQUARE_RE: Regex = Regex::new(r"^(?P<file>[a-h])(?P<rank>[1-8])$").unwrap();
    }

    let caps = SQUARE_RE.captures(square).unwrap();

    // convert tile to char
    // This ugly
    let file = caps["file"].chars().last().unwrap();
    // convert rank to u8
    let rank =caps["rank"].parse::<usize>().unwrap();

    return Square::new(file, rank)
}

pub fn s(s:&str) -> String {
    s.to_string()
}


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn str_to_square_should_return_square() {
        let str = "e4";
        let square = str_to_square(str);
        let expected = Square::new('e', 4);
        assert_eq!(square, expected);
    }
}