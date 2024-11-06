mod player;
use player::Player;

pub struct Team {
    players: Vec<Player>,
    team_type: TeamType,
    pub runs: u32,
    pub wickets: u32,
}

impl Team {
    pub fn new(team_type: TeamType) -> Self {
        Team {
            players: vec![],
            team_type,
            runs: 0,
            wickets: 0,
        }
    }

    pub fn new_with_players(team_type: TeamType, players: Vec<Player>) -> Self {
        Team {
            players,
            team_type,
            runs: 0,
            wickets: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum TeamType {
    A,
    B,
}
