use regex::Regex;

use crate::engine_core::square::Square;

#[allow(dead_code)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn str_to_square_should_return_square() {
        let str = "e4";
        let square = str_to_square(str);
        let expected = Square::new("e", 4);
        assert_eq!(square, expected);
    }
}
