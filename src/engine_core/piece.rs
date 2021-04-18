/// Helper enum to represent piece color
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Color {
    Black,
    White,
    None
}

/// Enum that represents a single Piece
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

/// Returns the string representation of the given Color
pub fn color_to_str(col: &Color) -> &'static str {
    match col {
        Color::Black => &"Black",
        Color::White => &"White",
        Color::None => &"None",
    }
}

/// Get string representation of the Piece.
/// # <'p> means that the strings lifetime is bound
/// # the lifetime of the argument Piece
pub fn piece_to_str<'p>(piece: &'p Piece) -> &'p str {
    match piece {
        Piece::King(c) => &format!("{} King", c),
        Piece::Queen(c) => &format!("{} Queen", c),
        Piece::Rook(c) => &format!("{} Rook", c),
        Piece::Bishop(c) => &format!("{} Bishop", c),
        Piece::Knight(c) => &format!("{} Knight", c),
        Piece::Pawn(c) => &format!("{} Pawn", c),
        Piece::Empty => &"Empty",
        Piece::Error => &"!ERROR!",
    }
}

/// Helper function to create Pieces. Should this be a macro?
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

/// Returns the PNG/FEN notation character for a given Piece
/// # These strings are compiled into the binary it self
/// # so they have static lifetime by definition
pub fn p_to_notation(p: &Piece) -> &'static str {
    match p {
        Piece::King(Color::White)   => &"K",
        Piece::Queen(Color::White)  => &"Q",
        Piece::Rook(Color::White)   => &"R",
        Piece::Bishop(Color::White) => &"B",
        Piece::Knight(Color::White) => &"N",
        Piece::Pawn(Color::White)   => &"P",
        Piece::King(Color::Black)   => &"k",
        Piece::Queen(Color::Black)  => &"q",
        Piece::Rook(Color::Black)   => &"r",
        Piece::Bishop(Color::Black) => &"b",
        Piece::Knight(Color::Black) => &"n",
        Piece::Pawn(Color::Black)   => &"p",
        Piece::Empty                => &" ",
        _                           => &"!"
    }
}

/// Returns the UTF-8 character representing given Piece
/// # These strings are compiled into the binary it self
/// # so they have static lifetime by definition
pub fn p_to_utf(p: &Piece) -> &'static str {
    match p {
        Piece::King(Color::White)   => &"♔",
        Piece::Queen(Color::White)  => &"♕",
        Piece::Rook(Color::White)   => &"♖",
        Piece::Bishop(Color::White) => &"♗",
        Piece::Knight(Color::White) => &"♘",
        Piece::Pawn(Color::White)   => &"♙",
        Piece::King(Color::Black)   => &"♚",
        Piece::Queen(Color::Black)  => &"♛",
        Piece::Rook(Color::Black)   => &"♜",
        Piece::Bishop(Color::Black) => &"♝",
        Piece::Knight(Color::Black) => &"♞",
        Piece::Pawn(Color::Black)   => &"♟︎",
        Piece::Empty                => &" ",
        _ => &"💩",
    }
}


#[cfg(test)]
mod test {
    use super::*;

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
        let expected = &"N";

        assert_eq!(&output, expected);
    }

    #[test]
    fn should_get_utf() {
        let expected = &"♟︎";
        let input = Piece::Pawn(Color::Black);
        let output = p_to_utf(&input);

        assert_eq!(&output, expected);
    }
}
