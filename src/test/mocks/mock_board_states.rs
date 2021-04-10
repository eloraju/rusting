use crate::engine_core::{
    piece::{
        Piece,
        p
    }
};

pub fn get_test_fen() -> String {
    "r1q1r1k1/1p1nbpp1/p1n1p1p1/3p2P1/3P1P1P/2P3B1/PP1N2B1/R2Q1RK1".to_string()
}

pub fn get_test_board_state() -> [[Piece;8]; 8] {
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
