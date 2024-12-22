use crate::state::Event;
use iced::widget::{container, text, Container};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Extras {
    wides: u32,
    no_balls: u32,
    byes: u32,
    leg_byes: u32,
    penalty_runs: u32,
}

impl Extras {
    // logic
    pub fn new() -> Self {
        Extras {
            wides: 0,
            no_balls: 0,
            byes: 0,
            leg_byes: 0,
            penalty_runs: 0,
        }
    }

    pub fn add_extra(&mut self, extra: &Extra) {
        match extra.extra_type {
            ExtraType::Wide => self.wides += extra.runs + 1,
            ExtraType::NoBall => self.no_balls += 1,
            ExtraType::Bye => self.byes += extra.runs,
            ExtraType::LegBye => self.leg_byes += extra.runs,
            ExtraType::PenaltyRuns => (),
        }
    }
}

impl Extras {
    // ui
    pub fn to_container(&self) -> Container<Event> {
        container(text(format!(
            "W: {wides}, NB: {no_balls}, B: {byes}, LB: {leg_byes}",
            wides = self.wides,
            no_balls = self.no_balls,
            byes = self.byes,
            leg_byes = self.leg_byes
        )))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Extra {
    pub runs: u32,
    pub extra_type: ExtraType,
}

impl Extra {
    pub fn new(runs: u32, extra_type: ExtraType) -> Self {
        Self { runs, extra_type }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, Display, EnumIter)]
pub enum ExtraType {
    Wide,
    #[strum(to_string = "No ball")]
    NoBall,
    Bye,
    #[strum(to_string = "Leg bye")]
    LegBye,
    #[strum(to_string = "Penalty runs")]
    PenaltyRuns,
}
