pub mod player;

use crate::state::game_state::extras::{Extra, ExtraType, Extras};
use crate::state::game_state::overs::Overs;
use player::Player;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Team {
    pub players: Vec<Rc<RefCell<Player>>>,
    pub team_name: String,
    pub runs: u32,
    pub wickets: u32,
    pub overs: Overs,
    pub extras: Extras,
}

impl Team {
    pub fn new() -> Self {
        Team {
            players: vec![],
            team_name: String::new(),
            runs: 0,
            wickets: 0,
            overs: Overs::new(),
            extras: Extras::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(Rc::new(RefCell::new(player)));
    }

    pub fn add_extra(&mut self, extra: &Extra) {
        let runs: u32;

        match extra.extra_type {
            ExtraType::Wide | ExtraType::NoBall => runs = extra.runs + 1,
            _ => runs = extra.runs,
        }

        self.runs += runs;
        self.extras.add_extra(extra);
    }

    pub fn next_bowling_order(&self) -> usize {
        self.players
            .iter()
            .map(|player| player.borrow().bowling_order)
            .filter(|bowling_order| bowling_order.as_ref() != None)
            .map(|bowling_order| bowling_order.unwrap().clone())
            .max()
            .expect("There should be a next bowling order if this function has been called")
            + 1
    }

    pub fn bowled_players_in_order(&self) -> Vec<Rc<RefCell<Player>>> {
        let mut players: Vec<Rc<RefCell<Player>>> = self
            .players
            .iter()
            .filter(|player| player.borrow().bowling_order != None)
            .map(|player| Rc::clone(player))
            .collect();

        players.sort_by(|a, b| {
            a.borrow()
                .bowling_order
                .unwrap()
                .cmp(&b.borrow().bowling_order.unwrap())
        });
        players
    }

    pub fn not_bowled_players(&self) -> Vec<Rc<RefCell<Player>>> {
        self.players
            .iter()
            .filter(|player| player.borrow().bowling_order == None)
            .map(|player| Rc::clone(player))
            .collect()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum TeamType {
    A,
    B,
}
