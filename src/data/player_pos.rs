use serde::{Serialize, Deserialize};

use super::team::Team;

#[derive(Serialize, Deserialize)]
pub struct PlayerPos {
    pos: (usize, usize),
    team: Team
}

impl PlayerPos {
    pub fn new(pos: (usize, usize), team: Team) -> Self { Self { pos, team } }
}