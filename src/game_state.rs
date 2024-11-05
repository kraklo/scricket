mod event;
mod team;

use event::Event;
use team::Team;

pub struct GameState {
    team_a: Team,
    team_b: Team,
    events: Vec<Event>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            team_a: Team::new(),
            team_b: Team::new(),
            events: vec![],
        }
    }
}

struct Overs {
    overs: u32,
    balls: u32,
}

impl Overs {
    fn new() -> Overs {
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
    fn new() -> Extras {
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
