use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Team {
    White,
    Red
}

impl Team {
    pub fn new(num: usize) -> Self {
        match num {
            0 => Self::White,
            1 => Self::Red,
            _ => panic!("Not a valid team number!")
        }
    }
}