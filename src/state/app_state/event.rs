#[derive(Debug, Clone)]
pub enum AppEvent {
    FirstNameChanged(String),
    LastNameChanged(String),
    SubmitName,
    SubmitTeam,
    LoadGame,
    NewGame,
    BatterSelected(u32),
    SubmitBatter,
    BowlerSelected(u32),
    SubmitBowler,
}
