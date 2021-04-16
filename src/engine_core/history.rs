use super::{
    state::State,
};

pub struct History<BoardType>{
    current_state: usize,
    states: Vec<State<BoardType>>
}

impl<BoardType> History<BoardType> {
    pub fn new(init_state: State<BoardType>) -> Self {
        let mut inst = Self {
            current_state: 0,
            states: Vec::new(),
        };

        inst.states.push(init_state);
        return inst;
    }

    pub fn state(&self) -> &State<BoardType> {
        &self.states[self.current_state]
    }

    pub fn next(&mut self) -> Option<&State<BoardType>> {
        let next = &self.current_state + 1;
        return self.go_to(next);
    }

    pub fn prev(&mut self) -> Option<&State<BoardType>> {
        let next = &self.current_state - 1;
        return self.go_to(next);
    }


    pub fn go_to(&mut self, index: usize) -> Option<&State<BoardType>> {
        let next_state = self.states.get(index);
        if next_state.is_some() {
            self.current_state = index;
        }

        return next_state;
    }

    // Mutation galore...
    pub fn advance(&mut self, new_state: State<BoardType>) {
        self.states.push(new_state);
        self.current_state = self.states.len()-1;
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{engine_core::
        {
            piece::{Color, Piece}
        },
        test::{
            mocks::{
                mock_board_states::{
                    init_board_state,
                    e4_board_state,
                    e5_board_state,
                }
            }
        }
    };

    #[test]
    fn should_create_history() {
        let board = init_board_state();
        let init_state = State::new(board);
        let hist = History::<[[Piece;8];8]>::new(init_state);

        assert_eq!(hist.current_state, 0);
        assert_eq!(hist.states.len(), 1);
        assert_eq!(hist.state().turn_count, 0);
    }

    #[test]
    fn should_advance_state() {
       let init_board = init_board_state();
       let init_state = State::new(init_board);
       let mut history = History::new(init_state) ;

       let e4 = e4_board_state();
       let new_state = State::new(e4);
       history.advance(new_state);

       let correct_piece = Piece::Pawn(Color::White);
       // e4 = [3][4]
       let piece_at_e4 = history.state().board[3][4];

       assert_eq!(piece_at_e4, correct_piece);
    }

    #[test]
    fn should_advance_and_get_previous_state() {
       let init_board = init_board_state();
       let init_state = State::new(init_board);
       let mut history = History::new(init_state) ;

       let e4 = e4_board_state();
       let new_state = State::new(e4);
       history.advance(new_state);

       history.prev();

       let piece_at_e4 = history.state().board[3][4];

       assert_eq!(history.current_state, 0);
       assert_eq!(history.state().board, init_board);
       assert_eq!(piece_at_e4, Piece::Empty);
    }

    #[test]
    fn should_be_able_to_navigate_states() {
       let init_board = init_board_state();
       let init_state = State::new(init_board);
       let mut history = History::new(init_state) ;

       let e4 = e4_board_state();
       let e4_state = State::new(e4);
       let e5 = e5_board_state();
       let e4e5_state = State::new(e5);

       history.advance(e4_state);
       history.advance(e4e5_state);
       history.prev();
       history.prev();

       assert_eq!(init_board, history.state().board);
       assert_eq!(history.current_state, 0);

       history.next();

       assert_eq!(e4, history.state().board);
       assert_eq!(history.current_state, 1);
    }
    #[test]
    fn should_get_e4_state() {
       let init_board = init_board_state();
       let init_state = State::new(init_board);
       let mut history = History::new(init_state) ;

       let e4 = e4_board_state();
       let e4_state = State::new(e4);
       let e5 = e5_board_state();
       let e4e5_state = State::new(e5);
       let d4 = e5_board_state();
       let e4e5d4_state = State::new(d4);

       history.advance(e4_state);
       history.advance(e4e5_state);
       history.advance(e4e5d4_state);

       history.go_to(1);

       assert_eq!(history.state().board, e4);
    }
}
