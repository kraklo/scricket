use super::TeamType;

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
