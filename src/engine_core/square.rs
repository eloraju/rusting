#[derive(Debug)]
#[derive(PartialEq)]
pub struct Square {
    pub file: String,
    pub rank: usize,
}

impl Square {
    pub fn new(file: &str, rank: usize) -> Self {
        Self {
            file: file.to_string(),
            rank
        }
    }
}
