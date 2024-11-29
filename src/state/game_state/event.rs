use super::{Player, TeamType};
use crate::state::app_state::event::AppEvent;
use iced::widget::{container, text, Container};

#[derive(Debug, Clone)]
pub enum Event {
    AppEvent(AppEvent),
    GameEvent(GameEvent),
}

#[derive(Debug, Clone)]
pub enum GameEvent {
    Runs(u32),
    Extra(Extra),
    Wicket,
    StartOver,
    EndOver,
    StartInnings(TeamType),
    EndInnings(TeamType),
    SelectOnStrike(Player),
    SelectOffStrike(Player),
    SelectBowler(Player),
}

impl GameEvent {
    pub fn to_container(&self) -> Container<Event> {
        let container_text = match self {
            Self::Runs(runs) => format!(
                "{runs} run{plural}",
                plural = if *runs == 1 { "" } else { "s" }
            ),
            Self::Wicket => String::from("wicket"),
            _ => String::from("none"),
        };

        container(text(container_text))
    }
}

#[derive(Debug, Clone)]
struct Extra {
    runs: u32,
    extra_type: ExtraType,
}

#[derive(Debug, Clone)]
enum ExtraType {
    Wide,
    NoBall,
    Bye,
    LegBye,
    PenaltyRuns,
}
