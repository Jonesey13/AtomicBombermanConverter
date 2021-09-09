use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Block {
    Solid,
    Brick,
    Blank
}

impl Block {
    pub fn new(ch: char) -> Self {
        match ch {
            '#' => Self::Solid,
            ':' => Self::Brick,
            '.' => Self::Blank,
            _ => panic!("Not a valid brick character!")
        }
    }
}