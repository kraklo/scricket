use super::Event;
use super::{HowOut, Player, TeamType};
use iced::widget::{container, text, Container};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

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
            Self::Extra(extra) => String::from(format!("extra: {}", extra.extra_type.to_string())),
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

impl Extra {
    pub fn new(extra_type: ExtraType) -> Self {
        Extra {
            runs: 0,
            extra_type,
        }
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
