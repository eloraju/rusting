use super::{
    state::State,
    engine::Engine
};

pub struct History<T: Engine>{
    current_state: usize,
    states: Vec<State<T>>
}

impl<T> History<T> 
where T: Engine
{
    fn new(init_state: State<T>) -> Self {
        let mut inst = Self {
            current_state: 0,
            states: Vec::new(),
        };

        inst.states.push(init_state);
        return inst;
    }

    fn state(&self) -> &State<T> {
        &self.states[self.current_state]
    }

    fn next(&mut self) -> Option<&State<T>> {
        let next = &self.current_state + 1;
        return self.go_to(next);
    }

    fn prev(&mut self) -> Option<&State<T>> {
        let next = &self.current_state - 1;
        return self.go_to(next);
    }


    fn go_to(&mut self, index: usize) -> Option<&State<T>> {
        let next_state = self.states.get(index);
        if next_state.is_some() {
            self.current_state = index;
        }

        return next_state;
    }

    // Mutation galore...
    fn push(&mut self, new_state: State<T>) {
        self.states.push(new_state);
        self.current_state = self.states.len();
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::mock_engine::MockEngine;

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
