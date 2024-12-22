pub mod event;
pub mod extras;
pub mod overs;
pub mod team;
pub mod wickets;

use crate::state::{Event, Page};
use event::GameEvent;
use extras::ExtraType;
use iced::widget::{button, column, row, scrollable, text, Column, Row};
use iced::Element;
pub use team::player::{Player, PlayerType};
pub use team::{Team, TeamType};
use wickets::HowOut;

#[derive(Clone)]
pub struct GameState {
    pub team_a: Team,
    pub team_b: Team,
    pub batting_team: TeamType,
    pub events: Vec<GameEvent>,
    batter_a: Option<Player>,
    batter_b: Option<Player>,
    on_strike_batter: PlayerType,
    pub bowler: Option<Player>,
}

impl GameState {
    // ui
    pub fn update(&mut self, event: GameEvent) -> Option<Page> {
        self.add_event(event.clone());

        let mut page = None;

        match event {
            GameEvent::Runs(runs) => {
                self.add_runs(&runs);

                if self.is_end_over() {
                    self.end_over();
                    page = Some(Page::SelectBowler);
                }
            }
            GameEvent::Wicket(how_out) => {
                self.add_wicket(&how_out);

                if self.is_end_over() {
                    self.end_over();
                    page = Some(Page::SelectBowler);
                }
            }
            GameEvent::SelectOnStrike(player) => {
                let mut batter = self.batting_team_mut().take_player(player);
                let team = self.batting_team();

                if team.wickets == 0 {
                    batter.batting_order = Some(0);
                } else {
                    batter.batting_order = Some(team.wickets + 1);
                }

                match self.on_strike_batter {
                    PlayerType::A => self.batter_a = Some(batter),
                    PlayerType::B => self.batter_b = Some(batter),
                }
            }
            GameEvent::SelectOffStrike(player) => {
                let mut batter = self.batting_team_mut().take_player(player);
                let team = self.batting_team();

                if team.wickets == 0 {
                    batter.batting_order = Some(0);
                } else {
                    batter.batting_order = Some(team.wickets + 1);
                }

                match self.on_strike_batter {
                    PlayerType::A => self.batter_b = Some(batter),
                    PlayerType::B => self.batter_a = Some(batter),
                }
            }
            GameEvent::SelectBowler(player) => {
                let bowling_order = 0; // TODO: find a way to calculate this

                if let Some(bowler) = self.bowler.clone() {
                    self.bowling_team_mut().put_player(bowler);
                }

                let mut bowler = self.bowling_team_mut().take_player(player);
                bowler.bowling_order = Some(bowling_order);
                self.bowler = Some(bowler);
            }
            GameEvent::SubmitTeam => self.change_team(),
            GameEvent::AddPlayer(player) => self.add_player(player),
            GameEvent::Extra(extra) => {
                let mut batting_team = self.batting_team_mut().to_owned();
                let mut batter = self.on_strike_batter_mut().unwrap().to_owned();
                let mut bowler = self.bowler.as_ref().unwrap().to_owned();

                bowler.add_extra(&extra);
                batting_team.add_extra(&extra);

                match extra.extra_type {
                    ExtraType::NoBall => {
                        batter.runs_scored += extra.runs;
                        batter.balls_faced += 1;
                    }
                    ExtraType::Bye | ExtraType::LegBye => {
                        batter.balls_faced += 1;
                        batting_team.overs.add_ball();
                    }
                    _ => (),
                }

                self.set_batting_team(batting_team);
                self.set_on_strike_batter(Some(batter));
                self.bowler = Some(bowler);

                if extra.runs % 2 == 1 {
                    self.change_strike();
                }
            }
            _ => (),
        }

        page
    }

    pub fn view(&self) -> Element<Event> {
        let team = self.batting_team();

        let mut content = column![
            text(format!(
                "{team_name}: {wickets}/{runs}",
                team_name = team.team_name,
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

        content = content.push(team.extras.to_container());

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
            button("wicket").on_press(Event::ChangePage(Page::SelectWicket)),
            button("extra").on_press(Event::ChangePage(Page::SelectExtra)),
            button("Save Game").on_press(Event::SaveGame),
        ]);
        content = content.push(scrollable(self.event_column()));

        content.into()
    }

    fn event_column(&self) -> Column<Event> {
        let mut column = Column::new();

        for event in &self.events {
            if let Some(event_container) = event.to_container() {
                column = column.push(event_container);
            }
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

    pub fn from_events(events: Vec<GameEvent>) -> Self {
        let mut game_state = GameState::new();

        for event in events {
            game_state.update(event);
        }

        game_state
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

    fn set_batting_team(&mut self, team: Team) {
        match self.batting_team {
            TeamType::A => self.team_a = team,
            TeamType::B => self.team_b = team,
        }
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

    pub fn add_player(&mut self, player: Player) {
        let team = self.batting_team_mut();
        team.add_player(player);
    }

    pub fn team_length(&self) -> usize {
        let team = self.batting_team();

        team.players.len()
    }

    pub fn change_team(&mut self) {
        match self.batting_team {
            TeamType::A => self.batting_team = TeamType::B,
            TeamType::B => self.batting_team = TeamType::A,
        };
    }

    fn change_strike(&mut self) {
        self.on_strike_batter = match self.on_strike_batter {
            PlayerType::A => PlayerType::B,
            PlayerType::B => PlayerType::A,
        }
    }

    fn add_runs(&mut self, runs: &u32) {
        let on_strike_batter = self
            .on_strike_batter_mut()
            .expect("A player should be on strike when add_runs is callled");

        on_strike_batter.balls_faced += 1;
        on_strike_batter.runs_scored += runs;

        let bowler = self
            .bowler
            .as_mut()
            .expect("A player should be bowling when add_runs is callled");

        bowler.overs_bowled.add_ball_bowler();
        bowler.runs_conceded += runs;

        let team = self.batting_team_mut();
        team.runs += runs;
        team.overs.add_ball();

        if runs % 2 == 1 {
            self.change_strike();
        }
    }

    fn add_wicket(&mut self, how_out: &HowOut) {
        let mut player = self
            .on_strike_batter_mut()
            .expect("There should be an on strike batter when a wicket occurs")
            .to_owned();
        player.how_out = how_out.clone();

        let team = self.batting_team_mut();
        team.wickets += 1;
        team.overs.add_ball();
        team.put_player(player.to_owned());

        let bowler = self
            .bowler
            .as_mut()
            .expect("There should be a bowler when a wicket occurs");
        bowler.wickets_taken += 1;
        bowler.overs_bowled.add_ball_bowler();

        self.set_on_strike_batter(None);
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

    pub fn set_on_strike_batter(&mut self, batter: Option<Player>) {
        match self.on_strike_batter {
            PlayerType::A => self.batter_a = batter,
            PlayerType::B => self.batter_b = batter,
        }
    }

    fn is_end_over(&self) -> bool {
        self.batting_team().overs.balls >= 6
    }

    fn end_over(&mut self) {
        if let Some(bowler) = self.bowler.clone() {
            self.bowling_team_mut().put_player(bowler);
            self.bowler = None;
        }

        self.batting_team_mut().overs.end_over();
        self.change_strike();
        self.add_event(GameEvent::EndOver);
    }
}

pub enum ReplaceBatter {
    OnStrike,
    OffStrike,
}
