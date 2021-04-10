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
    pub fn push(&mut self, new_state: State<BoardType>) {
        self.states.push(new_state);
        self.current_state = self.states.len();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test::{
            mocks::{
                mock_engine::MockEngine
            },
        },
        engine_core::engine::Engine,
    };

    #[test]
    fn should_create_history() {
        let board = MockEngine::new();
        let init_state = State::new(board);
        let hist = History::<MockEngine>::new(init_state);

        assert_eq!(hist.current_state, 0);
        assert_eq!(hist.states.len(), 1);
        assert_eq!(hist.state().turn_count, 0);
    }

}
