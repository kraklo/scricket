pub mod player;

use super::Overs;
use player::Player;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct Team {
    pub players: Vec<Option<Player>>,
    pub team_name: String,
    pub runs: u32,
    pub wickets: u32,
    pub overs: Overs,
}

impl Team {
    pub fn new() -> Self {
        Team {
            players: vec![],
            team_name: String::new(),
            runs: 0,
            wickets: 0,
            overs: Overs::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(Some(player));
    }

    pub fn player_from_order(&self, order: u32) -> Option<Player> {
        for player in &self.players {
            if let Some(player) = player {
                if player.order == order {
                    return Some(player.clone());
                }
            }
        }

        None
    }

    pub fn take_player(&mut self, player: Player) -> Player {
        let mut found_player_index: Option<usize> = None;

        for (i, team_player) in self.players.iter().enumerate() {
            if let Some(team_player) = team_player {
                if player == *team_player {
                    found_player_index = Some(i);
                }
            }
        }

        let found_player_index = found_player_index.expect("Player should exist");
        std::mem::replace(&mut self.players[found_player_index], None)
            .expect("Should be replacing a not None player")
    }

    pub fn put_player(&mut self, player: Player) {
        let index = player.order as usize;

        if self.players[index] != None {
            panic!("There should not be a player at {}", index);
        }

        self.players[index] = Some(player);
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum TeamType {
    A,
    B,
}
