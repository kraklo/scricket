use super::TeamType;
use iced::widget::{container, text, Container};

#[derive(Debug, Clone)]
pub enum Event {
    Runs(u32),
    Extra(Extra),
    Wicket,
    StartOver,
    EndOver,
    StartInnings(TeamType),
    EndInnings(TeamType),
}

impl Event {
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
