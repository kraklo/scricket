use crate::state::game_state::extras::{Extra, ExtraType, Extras};
use crate::state::game_state::overs::Overs;
use crate::state::game_state::wickets::{HowOut, WicketDetails};
use crate::state::Event;
use iced::widget::{container, text, Container};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Player {
    first_name: String,
    last_name: String,
    pub how_out: HowOut,
    pub wicket_details: Option<WicketDetails>,
    pub runs_scored: u32,
    pub balls_faced: u32,
    pub runs_conceded: u32,
    pub wickets_taken: u32,
    pub overs_bowled: Overs,
    extras: Extras,
    pub order: usize,
    pub batting_order: Option<usize>,
    pub bowling_order: Option<usize>,
}

impl Player {
    // logic
    pub fn new(first_name: &str, last_name: &str, order: usize) -> Self {
        Player {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            how_out: HowOut::DidNotBat,
            wicket_details: None,
            runs_scored: 0,
            balls_faced: 0,
            runs_conceded: 0,
            wickets_taken: 0,
            overs_bowled: Overs::new(),
            extras: Extras::new(),
            order,
            batting_order: None,
            bowling_order: None,
        }
    }

    pub fn add_extra(&mut self, extra: &Extra) {
        match extra.extra_type {
            ExtraType::Bye | ExtraType::LegBye => self.overs_bowled.add_ball_bowler(),
            _ => (),
        }

        self.extras.add_extra(extra);
    }
}

impl Player {
    // views
    pub fn to_container<'a>(self) -> Container<'a, Event> {
        container(text(self.to_string()))
    }

    pub fn to_batting_container<'a>(self) -> Container<'a, Event> {
        container(text(format!(
            "{name}: {runs} ({balls})",
            name = self.to_string(),
            runs = self.runs_scored,
            balls = self.balls_faced,
        )))
    }

    pub fn to_bowling_container<'a>(self) -> Container<'a, Event> {
        container(text(format!(
            "{name}: {wickets}/{runs} ({overs}.{balls})",
            name = self.to_string(),
            wickets = self.wickets_taken,
            runs = self.runs_conceded,
            overs = self.overs_bowled.overs,
            balls = self.overs_bowled.balls,
        )))
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.first_name, self.last_name)
    }
}

#[derive(Clone, PartialEq)]
pub enum PlayerType {
    A,
    B,
}
