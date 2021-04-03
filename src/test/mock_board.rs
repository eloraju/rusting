use crate::boards::board::Board;


    struct MockBoard {

    }

    impl Board for TestBoard {
        fn new() -> Self {
        todo!()
        }

        fn from_fen(fen_str: &str) -> Self {
        todo!()
    }

        fn from_pgn(fen_str: &str) -> Self {
        todo!()
    }

        fn from_8x8_str(str: &str) -> Self {
        todo!()
    }

        fn to_fen() -> String {
        todo!()
    }

        fn to_pgn() -> String {
        todo!()
    }

        fn to_8x8_str() -> String {
        todo!()
    }

        fn get(&self, square: crate::boards::board::Square) -> crate::boards::board::Occupant {
        todo!()
    }

        fn print(&self) {
        todo!()
    }
    }