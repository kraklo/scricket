pub mod event;
pub mod team;

use event::{Event, GameEvent};
use iced::widget::{button, column, row, scrollable, text, Column, Row};
use iced::Element;
use serde::{Deserialize, Serialize};
pub use team::player::{Player, PlayerType};
pub use team::{Team, TeamType};

#[derive(Clone, Deserialize, Serialize)]
pub struct GameState {
    pub team_a: Team,
    pub team_b: Team,
    pub batting_team: TeamType,
    events: Vec<GameEvent>,
    batter_a: Option<Player>,
    batter_b: Option<Player>,
    on_strike_batter: PlayerType,
    pub bowler: Option<Player>,
}

impl GameState {
    // ui
    pub fn update(&mut self, event: GameEvent) {
        match event {
            GameEvent::Runs(runs) => self.add_runs(runs),
            GameEvent::Wicket => self.team_a.wickets += 1,
            GameEvent::SelectOnStrike(ref player) => {
                let batter = Some(self.batting_team_mut().take_player(player.clone()));
                match self.on_strike_batter {
                    PlayerType::A => self.batter_a = batter,
                    PlayerType::B => self.batter_b = batter,
                }
            }
            GameEvent::SelectOffStrike(ref player) => {
                let batter = Some(self.batting_team_mut().take_player(player.clone()));
                match self.on_strike_batter {
                    PlayerType::A => self.batter_b = batter,
                    PlayerType::B => self.batter_a = batter,
                }
            }
            GameEvent::SelectBowler(ref player) => {
                self.bowler = Some(self.bowling_team_mut().take_player(player.clone()))
            }
            _ => (),
        }

        self.add_event(event);
    }

    pub fn view(&self) -> Element<Event> {
        let team = self.batting_team();

        let mut content = column![
            text(format!(
                "{wickets}/{runs}",
                wickets = team.wickets,
                runs = team.runs
            )),
            text(format!(
                "Overs: {overs}.{balls}",
                overs = team.overs.overs,
                balls = team.overs.balls
            ))
        ];

        if let Some(player) = &self.batter_a {
            let mut batting_container = Row::<Event>::new();
            if self.on_strike_batter == PlayerType::A {
                batting_container = batting_container.push(text("*"));
            }
            batting_container = batting_container.push(player.to_batting_container());
            content = content.push(batting_container);
        }

        if let Some(player) = &self.batter_b {
            let mut batting_container = Row::<Event>::new();
            if self.on_strike_batter == PlayerType::B {
                batting_container = batting_container.push(text("*"));
            }
            batting_container = batting_container.push(player.to_batting_container());
            content = content.push(batting_container);
        }

        if let Some(player) = &self.bowler {
            content = content.push(player.to_bowling_container());
        }

        content = content.push(row![
            button("0").on_press(Event::GameEvent(GameEvent::Runs(0))),
            button("1").on_press(Event::GameEvent(GameEvent::Runs(1))),
            button("2").on_press(Event::GameEvent(GameEvent::Runs(2))),
            button("3").on_press(Event::GameEvent(GameEvent::Runs(3))),
            button("4").on_press(Event::GameEvent(GameEvent::Runs(4))),
            button("6").on_press(Event::GameEvent(GameEvent::Runs(6))),
            button("wicket").on_press(Event::GameEvent(GameEvent::Wicket)),
        ]);
        content = content.push(scrollable(self.event_column()));

        content.into()
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
            if let Some(player) = player {
                column = column.push(player.to_container());
            }
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
            batter_a: None,
            batter_b: None,
            on_strike_batter: PlayerType::A,
            bowler: None,
        }
    }

    pub fn batting_team(&self) -> &Team {
        let team = match self.batting_team {
            TeamType::A => &self.team_a,
            TeamType::B => &self.team_b,
        };

        team
    }

    pub fn batting_team_mut(&mut self) -> &mut Team {
        let team = match self.batting_team {
            TeamType::A => &mut self.team_a,
            TeamType::B => &mut self.team_b,
        };

        team
    }

    pub fn bowling_team(&self) -> &Team {
        let team = match self.batting_team {
            TeamType::A => &self.team_b,
            TeamType::B => &self.team_a,
        };

        team
    }

    pub fn bowling_team_mut(&mut self) -> &mut Team {
        let team = match self.batting_team {
            TeamType::A => &mut self.team_b,
            TeamType::B => &mut self.team_a,
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
        self.on_strike_batter = match self.on_strike_batter {
            PlayerType::A => PlayerType::B,
            PlayerType::B => PlayerType::A,
        }
    }

    fn add_runs(&mut self, runs: u32) {
        let on_strike_batter = self
            .on_strike_batter_mut()
            .expect("A player should be on strike when add_runs is callled");

        on_strike_batter.balls_faced += 1;
        on_strike_batter.runs_scored += runs;

        let bowler = self
            .bowler
            .as_mut()
            .expect("A player should be bowling when add_runs is callled");

        bowler.overs_bowled.add_ball();
        bowler.runs_conceded += runs;

        let team = self.batting_team_mut();
        team.runs += runs;
        team.overs.add_ball();

        if runs % 2 == 1 {
            self.change_strike();
        }
    }

    pub fn batter_to_replace(&self) -> Option<ReplaceBatter> {
        if (self.on_strike_batter == PlayerType::A && self.batter_a == None)
            || (self.on_strike_batter == PlayerType::B && self.batter_b == None)
        {
            return Some(ReplaceBatter::OnStrike);
        }

        if (self.on_strike_batter == PlayerType::B && self.batter_a == None)
            || (self.on_strike_batter == PlayerType::A && self.batter_b == None)
        {
            return Some(ReplaceBatter::OffStrike);
        }

        None
    }

    pub fn on_strike_batter_mut(&mut self) -> Option<&mut Player> {
        match self.on_strike_batter {
            PlayerType::A => self.batter_a.as_mut(),
            PlayerType::B => self.batter_b.as_mut(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Retired {
    NotOut,
    Hurt,
}

pub enum ReplaceBatter {
    OnStrike,
    OffStrike,
}
