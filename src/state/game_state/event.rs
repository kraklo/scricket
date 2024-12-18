use super::Event;
use super::{HowOut, Player, TeamType};
use iced::widget::{container, text, Container};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GameEvent {
    Runs(u32),
    Extra(Extra),
    Wicket(HowOut),
    StartOver,
    EndOver,
    StartInnings(TeamType),
    EndInnings(TeamType),
    SelectOnStrike(Player),
    SelectOffStrike(Player),
    SelectBowler(Player),
    AddPlayer(Player),
    SubmitTeam,
}

impl GameEvent {
    pub fn to_container(&self) -> Option<Container<Event>> {
        let container_text = match self {
            Self::Runs(runs) => format!(
                "{runs} run{plural}",
                plural = if *runs == 1 { "" } else { "s" }
            ),
            Self::Wicket(how_out) => String::from(format!("wicket: {}", how_out.to_string())),
            _ => return None,
        };

        Some(container(text(container_text)))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Extra {
    runs: u32,
    extra_type: ExtraType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
enum ExtraType {
    Wide,
    NoBall,
    Bye,
    LegBye,
    PenaltyRuns,
}
