mod event;
mod team;

pub use event::Event;
use iced::widget::{button, column, container, row, scrollable, text, Column};
use iced::Element;
use team::{Team, TeamType};

pub struct GameState {
    pub team_a: Team,
    pub team_b: Team,
    events: Vec<Event>,
}

impl GameState {
    // ui
    pub fn update(&mut self, event: Event) {
        match event {
            Event::Runs(runs) => self.team_a.runs += runs,
            Event::Wicket => self.team_a.wickets += 1,
            _ => (),
        }

        self.add_event(event);
    }

    pub fn view(&self) -> Element<Event> {
        container(column![
            text(format!(
                "{wickets}/{runs}",
                wickets = self.team_a.wickets,
                runs = self.team_a.runs
            )),
            row![
                button("0").on_press(Event::Runs(0)),
                button("1").on_press(Event::Runs(1)),
                button("2").on_press(Event::Runs(2)),
                button("3").on_press(Event::Runs(3)),
                button("4").on_press(Event::Runs(4)),
                button("6").on_press(Event::Runs(6)),
                button("wicket").on_press(Event::Wicket),
            ],
            scrollable(self.event_column())
        ])
        .into()
    }

    fn event_column(&self) -> Column<Event> {
        let mut column = Column::new();

        for event in &self.events {
            column = column.push(event.to_container());
        }

        column
    }
}

impl GameState {
    // business logic
    pub fn new() -> Self {
        GameState {
            team_a: Team::new(TeamType::A),
            team_b: Team::new(TeamType::B),
            events: vec![],
        }
    }

    fn add_event(&mut self, event: Event) {
        self.events.push(event);
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
