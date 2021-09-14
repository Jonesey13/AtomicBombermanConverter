use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Powerup {
    bornwith: usize,
    has_override: bool,
    override_value: usize,
    forbidden: bool,
}

impl Powerup {
    pub fn new(
        bornwith: usize,
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
