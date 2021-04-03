pub trait Board {
    fn from_fen(fen_str: &str) -> Self;
    fn from_pgn(fen_str: &str) -> Self;
    fn from_8x8_str(str: &str) -> Self;
    fn to_fen() -> String;
    fn to_pgn() -> String;
    fn to_8x8_str() -> String;
    fn print(&self);
}
