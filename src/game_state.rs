mod event;
mod team;

pub use event::Event;
use team::{Team, TeamType};

pub struct GameState {
    pub team_a: Team,
    pub team_b: Team,
    events: Vec<Event>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            team_a: Team::new(TeamType::A),
            team_b: Team::new(TeamType::B),
            events: vec![],
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

struct Overs {
    overs: u32,
    balls: u32,
}

impl Overs {
    fn new() -> Self {
        Overs { overs: 0, balls: 0 }
    }
}

struct Extras {
    wides: u32,
    no_balls: u32,
    byes: u32,
    leg_byes: u32,
    penalty_runs: u32,
}

impl Extras {
    fn new() -> Self {
        Extras {
            wides: 0,
            no_balls: 0,
            byes: 0,
            leg_byes: 0,
            penalty_runs: 0,
        }
    }
}

enum HowOut {
    DidNotBat,
    NotOut,
    Bowled,
    Lbw,
    Caught,
    RunOut,
    Stumped,
    HitWicket,
    HitBallTwice,
    HandledBall,
    ObstructedField,
    TimedOut,
    Retired(Retired),
}

enum Retired {
    NotOut,
    Hurt,
}
