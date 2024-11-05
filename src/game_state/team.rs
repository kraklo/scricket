mod player;
use player::Player;

pub struct Team {
    players: Vec<Player>,
    runs: u32,
    wickets: u32,
}

impl Team {
    pub fn new() -> Team {
        Team {
            players: vec![],
            runs: 0,
            wickets: 0,
        }
    }

    pub fn new_with_players(players: Vec<Player>) -> Team {
        Team {
            players,
            runs: 0,
            wickets: 0,
        }
    }
}
