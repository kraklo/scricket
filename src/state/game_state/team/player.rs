use super::super::Event;
use super::super::{Extras, HowOut, Overs};
use iced::widget::{container, text, Container};

#[derive(Clone)]
pub struct Player {
    first_name: String,
    last_name: String,
    how_out: HowOut,
    pub runs_scored: u32,
    pub balls_faced: u32,
    pub runs_conceded: u32,
    pub overs_bowled: Overs,
    extras: Extras,
    batting_order: u32,
    bowling_order: Option<u32>,
}

impl Player {
    pub fn new(first_name: &str, last_name: &str, batting_order: u32) -> Self {
        Player {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            how_out: HowOut::DidNotBat,
            runs_scored: 0,
            balls_faced: 0,
            runs_conceded: 0,
            overs_bowled: Overs::new(),
            extras: Extras::new(),
            batting_order,
            bowling_order: None,
        }
    }
}

impl Player {
    pub fn to_container(&self) -> Container<Event> {
        container(text(format!(
            "{first_name} {last_name}",
            first_name = self.first_name,
            last_name = self.last_name
        )))
    }
}
