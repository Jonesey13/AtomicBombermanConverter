use serde::{Deserialize, Serialize};

use super::poweruptype::PowerupType;

#[derive(Serialize, Deserialize)]
pub struct Powerup {
    powerup_type: PowerupType,
    bornwith: bool,
    has_override: bool,
    override_value: usize,
    forbidden: bool,
}

impl Powerup {
    pub fn new(
        powerup_type: PowerupType,
        bornwith: bool,
        has_override: bool,
        override_value: usize,
        forbidden: bool,
    ) -> Self {
        Self {
            powerup_type,
            bornwith,
            has_override,
            override_value,
            forbidden,
        }
    }
}
