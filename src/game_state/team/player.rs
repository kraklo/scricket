use crate::game_state::{Extras, HowOut, Overs};

pub struct Player {
    first_name: String,
    last_name: String,
    how_out: HowOut,
    runs_scored: u32,
    balls_faced: u32,
    runs_conceded: u32,
    overs_bowled: Overs,
    extras: Extras,
}

impl Player {
    pub fn new(first_name: &str, last_name: &str) -> Player {
        Player {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            how_out: HowOut::DidNotBat,
            runs_scored: 0,
            balls_faced: 0,
            runs_conceded: 0,
            overs_bowled: Overs::new(),
            extras: Extras::new(),
        }
    }
}
