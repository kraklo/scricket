mod player;

use player::Player;

pub struct Team {
    pub players: Vec<Player>,
    team_name: String,
    pub runs: u32,
    pub wickets: u32,
}

impl Team {
    pub fn new() -> Self {
        Team {
            players: vec![],
            team_name: String::new(),
            runs: 0,
            wickets: 0,
        }
    }

    pub fn add_player(&mut self, first_name: &str, last_name: &str) {
        self.players.push(Player::new(first_name, last_name));
    }
}

#[derive(Debug, Clone)]
pub enum TeamType {
    A,
    B,
}
