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