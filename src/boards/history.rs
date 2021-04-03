use crate::Board;
pub struct History<T: Board> {
    current_state: usize,
    states: Vec<T>
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

    fn state(&self) -> &T {
        &self.states[self.current_state]
    }

    fn next(&self) -> Option<&T> {
        let next = &self.current_state + 1;
        return self.go_to(next);
    }

    fn prev(&self) -> Option<&T> {
        let next = &self.current_state - 1;
        return self.go_to(next);
    }


    fn go_to(&self, index: usize) -> Option<&T> {
        let next_state = &self.states.get(index);
        if next_state != None {
            &self.current_state = index;
        }

        return next_state;
    }
}

