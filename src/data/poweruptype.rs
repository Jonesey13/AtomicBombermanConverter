use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Hash)]
pub enum PowerupType {
    Bomb,
    Flame,
    Disease,
    BombKick,
    Speed,
    BombPunch,
    BombGrab,
    LineBomb,
    MaxFlame,
    TriggerBomb,
    BouncyBomb,
    SuperDisease,
    Random
}

impl PowerupType {
    pub fn new(num: usize) -> Self {
        match num {
            0 => Self::Bomb,
            1 => Self::Flame,
            2 => Self::Disease,
            3 => Self::BombKick,
            4 => Self::Speed,
            5 => Self::BombPunch,
            6 => Self::BombGrab,
            7 => Self::LineBomb,
            8 => Self::MaxFlame,
            9 => Self::TriggerBomb,
            10 => Self::BouncyBomb,
            11 => Self::SuperDisease,
            12 => Self::Random,
            _ => panic!("Invalid Powerup Type!")
        }
    }
}