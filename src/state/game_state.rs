pub mod event;
mod team;

use event::{Event, GameEvent};
use iced::widget::{button, column, container, row, scrollable, text, Column};
use iced::Element;
pub use team::{Team, TeamType};

pub struct GameState {
    pub team_a: Team,
    pub team_b: Team,
    events: Vec<GameEvent>,
    pub current_team: TeamType,
}

impl GameState {
    // ui
    pub fn update(&mut self, event: GameEvent) {
        // add ball if needed
        match event {
            GameEvent::Runs(_) | GameEvent::Wicket => {
                let team = self.current_team_mut();
                team.overs.add_ball();
            }
            _ => (),
        }

        // handle event otherwise
        match event {
            GameEvent::Runs(runs) => self.team_a.runs += runs,
            GameEvent::Wicket => self.team_a.wickets += 1,
            _ => (),
        }

        self.add_event(event);
    }

    pub fn view(&self) -> Element<Event> {
        let team = self.current_team();

        container(column![
            text(format!(
                "{wickets}/{runs}",
                wickets = team.wickets,
                runs = team.runs
            )),
            text(format!(
                "Overs: {overs}.{balls}",
                overs = team.overs.overs,
                balls = team.overs.balls
            )),
            row![
                button("0").on_press(Event::GameEvent(GameEvent::Runs(0))),
                button("1").on_press(Event::GameEvent(GameEvent::Runs(1))),
                button("2").on_press(Event::GameEvent(GameEvent::Runs(2))),
                button("3").on_press(Event::GameEvent(GameEvent::Runs(3))),
                button("4").on_press(Event::GameEvent(GameEvent::Runs(4))),
                button("6").on_press(Event::GameEvent(GameEvent::Runs(6))),
                button("wicket").on_press(Event::GameEvent(GameEvent::Wicket)),
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

    pub fn player_column(&self) -> Column<Event> {
        let team = self.current_team();

        let mut column = Column::new();

        for player in &team.players {
            column = column.push(player.to_container());
        }

        column
    }
}

impl GameState {
    // business logic
    pub fn new() -> Self {
        GameState {
            team_a: Team::new(),
            team_b: Team::new(),
            events: vec![],
            current_team: TeamType::A,
        }
    }

    fn current_team(&self) -> &Team {
        let team = match self.current_team {
            TeamType::A => &self.team_a,
            TeamType::B => &self.team_b,
        };

        team
    }

    fn current_team_mut(&mut self) -> &mut Team {
        let team = match self.current_team {
            TeamType::A => &mut self.team_a,
            TeamType::B => &mut self.team_b,
        };

        team
    }

    fn add_event(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn add_player(&mut self, first_name: &str, last_name: &str) {
        let team = self.current_team_mut();
        team.add_player(first_name, last_name);
    }

    pub fn team_length(&self) -> usize {
        let team = self.current_team();

        team.players.len()
    }

    pub fn change_team(&mut self) {
        match self.current_team {
            TeamType::A => self.current_team = TeamType::B,
            TeamType::B => self.current_team = TeamType::B,
        };
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

    fn add_ball(&mut self) {
        self.balls += 1;
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
