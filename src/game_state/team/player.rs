use super::super::Event;
use super::super::{Extras, HowOut, Overs};
use iced::widget::{container, text, Container};

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
    pub fn new(first_name: &str, last_name: &str) -> Self {
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

impl Player {
    pub fn to_container(&self) -> Container<Event> {
        container(text(format!(
            "{first_name} {last_name}",
            first_name = self.first_name,
            last_name = self.last_name
        )))
    }
}
