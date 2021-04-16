use crate::engine_core::{
    piece::{
        Piece,
        p
    },
};

use crate::array_engine::fen::input::fen_to_board;

pub fn get_test_fen() -> String {
    "r1q1r1k1/1p1nbpp1/p1n1p1p1/3p2P1/3P1P1P/2P3B1/PP1N2B1/R2Q1RK1".to_string()
}

pub fn get_mock_board_state() -> [[Piece;8]; 8] {
    [
        [p("R"),p(" "),p(" "),p("Q"),p(" "),p("R"),p("K"),p(" ")],
        [p("P"),p("P"),p(" "),p("N"),p(" "),p(" "),p("B"),p(" ")],
        [p(" "),p(" "),p("P"),p(" "),p(" "),p(" "),p("B"),p(" ")],
        [p(" "),p(" "),p(" "),p("P"),p(" "),p("P"),p(" "),p("P")],
        [p(" "),p(" "),p(" "),p("p"),p(" "),p(" "),p("P"),p(" ")],
        [p("p"),p(" "),p("n"),p(" "),p("p"),p(" "),p("p"),p(" ")],
        [p(" "),p("p"),p(" "),p("n"),p("b"),p("p"),p("p"),p(" ")],
        [p("r"),p(" "),p("q"),p(" "),p("r"),p(" "),p("k"),p(" ")],
    ]
}

pub fn init_board_state() -> [[Piece;8];8] {
    fen_to_board("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR")
}

pub fn e4_board_state() -> [[Piece;8];8] {
    fen_to_board("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR")
}

pub fn e5_board_state() -> [[Piece;8];8] {
    fen_to_board("rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR")
}

pub fn d4_board_state() -> [[Piece;8];8] {
    fen_to_board("rnbqkbnr/pppp1ppp/8/4p3/3PP3/8/PPP2PPP/RNBQKBNR")
}