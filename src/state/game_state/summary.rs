use serde::{Deserialize, Serialize};

use crate::state::game_state::overs::Overs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Summary {
    pub runs: u32,
    pub wickets: u32,
    pub overs: Overs,
}

impl Summary {
    pub fn new(runs: u32, wickets: u32, overs: Overs) -> Self {
        Self {
            runs,
            wickets,
            overs,
        }
    }
}
