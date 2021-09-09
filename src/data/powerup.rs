use serde::{Deserialize, Serialize};

use super::poweruptype::PowerupType;

#[derive(Serialize, Deserialize)]
pub struct Powerup {
    bornwith: bool,
    has_override: bool,
    override_value: usize,
    forbidden: bool,
}

impl Powerup {
    pub fn new(
        bornwith: bool,
        has_override: bool,
        override_value: usize,
        forbidden: bool,
    ) -> Self {
        Self {
            bornwith,
            has_override,
            override_value,
            forbidden,
        }
    }
}
