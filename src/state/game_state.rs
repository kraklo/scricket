pub mod event;
mod team;

use event::{Event, GameEvent};
use iced::widget::{button, column, container, row, scrollable, text, Column};
use iced::Element;
use team::player::Player;
pub use team::{Team, TeamType};

pub struct GameState {
    pub team_a: Team,
    pub team_b: Team,
    pub batting_team: TeamType,
    events: Vec<GameEvent>,
    on_strike_batter: Option<Player>,
    off_strike_batter: Option<Player>,
    bowler: Option<Player>,
}

impl GameState {
    // ui
    pub fn update(&mut self, event: GameEvent) {
        // handle event otherwise
        match event {
            GameEvent::Runs(runs) => self.add_runs(runs),
            GameEvent::Wicket => self.team_a.wickets += 1,
            _ => (),
        }

        self.add_event(event);
    }

    pub fn view(&self) -> Element<Event> {
        let team = self.batting_team();

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
        let team = self.batting_team();

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
            batting_team: TeamType::A,
            on_strike_batter: None,
            off_strike_batter: None,
            bowler: None,
        }
    }

    fn batting_team(&self) -> &Team {
        let team = match self.batting_team {
            TeamType::A => &self.team_a,
            TeamType::B => &self.team_b,
        };

        team
    }

    fn batting_team_mut(&mut self) -> &mut Team {
        let team = match self.batting_team {
            TeamType::A => &mut self.team_a,
            TeamType::B => &mut self.team_b,
        };

        team
    }

    fn add_event(&mut self, event: GameEvent) {
        self.events.push(event);
    }

    pub fn add_player(&mut self, first_name: &str, last_name: &str, batting_order: u32) {
        let team = self.batting_team_mut();
        team.add_player(first_name, last_name, batting_order);
    }

    pub fn team_length(&self) -> usize {
        let team = self.batting_team();

        team.players.len()
    }

    pub fn change_team(&mut self) {
        match self.batting_team {
            TeamType::A => self.batting_team = TeamType::B,
            TeamType::B => self.batting_team = TeamType::B,
        };
    }

    fn change_strike(&mut self) {
        std::mem::swap(&mut self.on_strike_batter, &mut self.off_strike_batter);
    }

    fn add_runs(&mut self, runs: u32) {
        let on_strike_batter = self
            .on_strike_batter
            .as_mut()
            .expect("A player should be on strike when add_runs is callled");

        let bowler = self
            .bowler
            .as_mut()
            .expect("A player should be bowling when add_runs is callled");

        bowler.overs_bowled.add_ball();
        bowler.runs_conceded += runs;

        on_strike_batter.balls_faced += 1;
        on_strike_batter.runs_scored += runs;

        let team = self.batting_team_mut();
        team.runs += runs;
        team.overs.add_ball();

        if runs % 2 == 1 {
            self.change_strike();
        }
    }
}

#[derive(Clone)]
pub struct Overs {
    overs: u32,
    balls: u32,
}

impl Overs {
    fn new() -> Self {
        Overs { overs: 0, balls: 0 }
    }

    fn add_ball(&mut self) {
        self.balls += 1;

        if self.balls == 6 {
            self.overs += 1;
            self.balls = 0;
        }
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
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

#[derive(Clone)]
enum Retired {
    NotOut,
    Hurt,
}
