use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{block::Block, player_pos::PlayerPos, powerup::Powerup, poweruptype::PowerupType};

#[derive(Serialize, Deserialize)]
pub struct Scheme {
    name: String,
    version: String,
    brick_density: usize,
    grid: Vec<Vec<Block>>,
    start_positions: Vec<PlayerPos>,
    powerups: HashMap<PowerupType, Powerup>,
}

impl Scheme {
    pub fn new(
        name: String,
        version: String,
        brick_density: usize,
        grid: Vec<Vec<Block>>,
        start_positions: Vec<PlayerPos>,
        powerups: HashMap<PowerupType, Powerup>,
    ) -> Self {
        Self {
            name,
            version,
            brick_density,
            grid,
            start_positions,
            powerups,
        }
    }
}
