use super::helpers::{char_to_piece, s};

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Color {
    Black,
    White,
    None
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum PieceEnum {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty
}
#[derive(Debug)]
#[derive(PartialEq)]
pub struct Piece {
    pub color: Color,
    pub piece: PieceEnum,
    pub empty: bool,
    pub notation: String,
    pub utf_piece: String
}

impl Piece {
    pub fn new(notation: &str) -> Self {
        match notation {
            "K" => Self {
                color: Color::White,
                piece: PieceEnum::King,
                empty: false,
                notation: s("K"),
                utf_piece: char_to_piece(notation),
            },

            "Q" => Self {
                color: Color::White,
                piece: PieceEnum::Queen,
                empty: false,
                notation: s("Q"),
                utf_piece: char_to_piece(notation),
            },

            "R" => Self {
                color: Color::White,
                piece: PieceEnum::Rook,
                empty: false,
                notation: s("R"),
                utf_piece: char_to_piece(notation),
            },

            "B" => Self {
                color: Color::White,
                piece: PieceEnum::Bishop,
                empty: false,
                notation: s("B"),
                utf_piece: char_to_piece(notation),
            },

            "N" => Self {
                color: Color::White,
                piece: PieceEnum::Knight,
                empty: false,
                notation: s("N"),
                utf_piece: char_to_piece(notation),
            },

            "P" => Self {
                color: Color::White,
                piece: PieceEnum::Pawn,
                empty: false,
                notation: s("P"),
                utf_piece: char_to_piece(notation),
            },

            "k" => Self {
                color: Color::Black,
                piece: PieceEnum::King,
                empty: false,
                notation: s("k"),
                utf_piece: char_to_piece(notation),
            },

            "q" => Self {
                color: Color::Black,
                piece: PieceEnum::Queen,
                empty: false,
                notation: s("q"),
                utf_piece: char_to_piece(notation),
            },

            "r" => Self {
                color: Color::Black,
                piece: PieceEnum::Rook,
                empty: false,
                notation: s("r"),
                utf_piece: char_to_piece(notation),
            },

            "b" => Self {
                color: Color::Black,
                piece: PieceEnum::Bishop,
                empty: false,
                notation: s("b"),
                utf_piece: char_to_piece(notation),
            },

            "n" => Self {
                color: Color::Black,
                piece: PieceEnum::Knight,
                empty: false,
                notation: s("n"),
                utf_piece: char_to_piece(notation),
            },

            "p" => Self {
                color: Color::Black,
                piece: PieceEnum::Pawn,
                empty: false,
                notation: s("p"),
                utf_piece: char_to_piece(notation),
            },

            " " => Self {
                color: Color::None,
                piece: PieceEnum::Empty,
                empty: true,
                notation: s(" "),
                utf_piece: char_to_piece(notation),
            },

            _ => Self {
                color: Color::None,
                piece: PieceEnum::Empty,
                empty: false,
                notation: s("!"),
                utf_piece: s("!")
            }
        }
    }
}

pub fn p(s: &str) -> Piece {
    Piece::new(s)
}