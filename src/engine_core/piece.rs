use super::helpers::s;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
    None
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Piece {
    King(Color),
    Queen(Color),
    Rook(Color),
    Bishop(Color),
    Knight(Color),
    Pawn(Color),
    Empty,
    Error
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", piece_to_str(&self))
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", color_to_str(self))
    }
}

pub fn color_to_str(col: &Color) -> String {
    match col {
        Color::Black => "Black".to_string(),
        Color::White => "White".to_string(),
        Color::None => "None".to_string(),
    }
}
pub fn piece_to_str(piece: &Piece) -> String {
    match piece {
        Piece::King(c) => format!("{} King", c),
        Piece::Queen(c) => format!("{} Queen", c),
        Piece::Rook(c) => format!("{} Rook", c),
        Piece::Bishop(c) => format!("{} Bishop", c),
        Piece::Knight(c) => format!("{} Knight", c),
        Piece::Pawn(c) => format!("{} Pawn", c),
        Piece::Empty => "Empty".to_string(),
        Piece::Error => "!ERROR!".to_string(),
    }
}

pub fn p(notation: &str) -> Piece {
    match notation {
        "K" => Piece::King(Color::White),
        "Q" => Piece::Queen(Color::White),
        "R" => Piece::Rook(Color::White),
        "B" => Piece::Bishop(Color::White),
        "N" => Piece::Knight(Color::White),
        "P" => Piece::Pawn(Color::White),
        "k" => Piece::King(Color::Black),
        "q" => Piece::Queen(Color::Black),
        "r" => Piece::Rook(Color::Black),
        "b" => Piece::Bishop(Color::Black),
        "n" => Piece::Knight(Color::Black),
        "p" => Piece::Pawn(Color::Black),
        " " => Piece::Empty,
        _ => Piece::Error,
    }
}

pub fn p_to_notation(p: &Piece) -> String {
    match p {
        Piece::King(Color::White)   => s("K"),
        Piece::Queen(Color::White)  => s("Q"),
        Piece::Rook(Color::White)   => s("R"),
        Piece::Bishop(Color::White) => s("B"),
        Piece::Knight(Color::White) => s("N"),
        Piece::Pawn(Color::White)   => s("P"),
        Piece::King(Color::Black)   => s("k"),
        Piece::Queen(Color::Black)  => s("q"),
        Piece::Rook(Color::Black)   => s("r"),
        Piece::Bishop(Color::Black) => s("b"),
        Piece::Knight(Color::Black) => s("n"),
        Piece::Pawn(Color::Black)   => s("p"),
        Piece::Empty                => s(" "),
        _                           => s("!")
    }
}

pub fn p_to_utf(p: &Piece) -> String {
    match p {
        Piece::King(Color::White)   => s("♔"),
        Piece::Queen(Color::White)  => s("♕"),
        Piece::Rook(Color::White)   => s("♖"),
        Piece::Bishop(Color::White) => s("♗"),
        Piece::Knight(Color::White) => s("♘"),
        Piece::Pawn(Color::White)   => s("♙"),
        Piece::King(Color::Black)   => s("♚"),
        Piece::Queen(Color::Black)  => s("♛"),
        Piece::Rook(Color::Black)   => s("♜"),
        Piece::Bishop(Color::Black) => s("♝"),
        Piece::Knight(Color::Black) => s("♞"),
        Piece::Pawn(Color::Black)   => s("♟︎"),
        Piece::Empty                => s(" "),
        _ => s("💩"),
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use crate::engine_core::helpers::s;

    #[test]
    fn should_create_piece_enum() {
        let input = "q";
        let expected = Piece::Queen(Color::Black);
        let output = p(input);

        assert_eq!(output, expected);
    }

    #[test]
    fn should_get_notation() {
        let input = Piece::Knight(Color::White);
        let output = p_to_notation(&input);
        let expected = s("N");

        assert_eq!(output, expected);
    }

    #[test]
    fn should_get_utf() {
        let expected = s("♟︎");
        let input = Piece::Pawn(Color::Black);
        let output = p_to_utf(&input);

        assert_eq!(output, expected);
    }
}