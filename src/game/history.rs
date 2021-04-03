use crate::Board;
use super::state::State;

pub struct History<T: Board>{
    current_state: usize,
    states: Vec<State<T>>
}

impl<T> History<T> 
where T: Board
{
    fn new() -> Self {
        Self {
            current_state: 0,
            states: Vec::new(),
        }
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
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::mock_board;

    fn should_create_history() {
        let hist = History<>::new();
    }
}