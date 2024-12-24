pub mod event;
pub mod extras;
pub mod overs;
pub mod team;
pub mod wickets;

use crate::state::{Event, Page};
use event::{GameEvent, GameEventHistory};
use extras::ExtraType;
use iced::widget::{button, column, row, scrollable, text, Column, Row};
use iced::Element;
use std::cell::RefCell;
use std::rc::Rc;
pub use team::player::{Player, PlayerType};
pub use team::{Team, TeamType};
use wickets::{HowOut, WicketDetails, WicketEvent};

#[derive(Clone)]
pub struct GameState {
    pub team_a: Team,
    pub team_b: Team,
    pub batting_team: TeamType,
    pub events: Vec<GameEvent>,
    event_history: Vec<GameEventHistory>,
    batter_a: Option<Rc<RefCell<Player>>>,
    batter_b: Option<Rc<RefCell<Player>>>,
    on_strike_batter: PlayerType,
    pub bowler: Option<Rc<RefCell<Player>>>,
    last_bowler: Option<usize>,
    pub last_last_bowler: Option<usize>,
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
            GameEvent::Wicket(wicket_event) => {
                self.add_wicket(&wicket_event);

                if self.batting_team().wickets == 10 {
                    self.update(GameEvent::EndInnings);
                    self.update(GameEvent::StartInnings(self.batting_team.clone()));
                    page = Some(Page::SelectBatter);
                } else if self.is_end_over() {
                    self.end_over();
                    page = Some(Page::SelectBowler);
                }
            }
            GameEvent::SelectOnStrike(player) => {
                let team = self.batting_team();
                let batter_ref = Rc::clone(&team.players[player]);
                let mut batter = batter_ref.borrow_mut();

                if team.wickets == 0 {
                    batter.batting_order = Some(0);
                } else {
                    batter.batting_order = Some((team.wickets + 1) as usize);
                }

                batter.how_out = HowOut::NotOut;

                match self.on_strike_batter {
                    PlayerType::A => self.batter_a = Some(Rc::clone(&batter_ref)),
                    PlayerType::B => self.batter_b = Some(Rc::clone(&batter_ref)),
                }
            }
            GameEvent::SelectOffStrike(player) => {
                let team = self.batting_team();
                let batter_ref = Rc::clone(&team.players[player]);
                let mut batter = batter_ref.borrow_mut();

                if team.wickets == 0 {
                    batter.batting_order = Some(0);
                } else {
                    batter.batting_order = Some((team.wickets + 1) as usize);
                }

                batter.how_out = HowOut::NotOut;

                match self.on_strike_batter {
                    PlayerType::A => self.batter_b = Some(Rc::clone(&batter_ref)),
                    PlayerType::B => self.batter_a = Some(Rc::clone(&batter_ref)),
                }
            }
            GameEvent::SelectBowler(player) => {
                let bowler_ref = Rc::clone(&self.bowling_team().players[player]);
                let bowler = bowler_ref.borrow();

                let bowling_order = match self.last_bowler {
                    Some(_) => {
                        if bowler.bowling_order == None {
                            self.bowling_team().next_bowling_order()
                        } else {
                            bowler.bowling_order.unwrap()
                        }
                    }
                    None => 0,
                };
                drop(bowler); // drop to borrow mut

                let mut bowler = bowler_ref.borrow_mut();
                bowler.bowling_order = Some(bowling_order);

                self.bowler = Some(Rc::clone(&bowler_ref));
            }
            GameEvent::SubmitTeam(team_name) => {
                self.batting_team_mut().team_name = team_name;
                self.change_team();
            }
            GameEvent::AddPlayer(player) => self.add_player(player),
            GameEvent::Extra(extra) => {
                let batter_ref = Rc::clone(&self.on_strike_batter().unwrap());
                let mut batter = batter_ref.borrow_mut();

                let bowler_ref = Rc::clone(&self.bowler.as_ref().unwrap());
                let mut bowler = bowler_ref.borrow_mut();

                bowler.add_extra(&extra);

                match extra.extra_type {
                    ExtraType::NoBall => {
                        batter.runs_scored += extra.runs;
                        batter.balls_faced += 1;
                    }
                    ExtraType::Bye | ExtraType::LegBye => {
                        batter.balls_faced += 1;
                    }
                    _ => (),
                }

                let batting_team = self.batting_team_mut();
                batting_team.add_extra(&extra);

                match extra.extra_type {
                    ExtraType::Bye | ExtraType::LegBye => {
                        batting_team.overs.add_ball();
                    }
                    _ => (),
                }

                if extra.runs % 2 == 1 {
                    self.change_strike();
                }
            }
            GameEvent::StartInnings(team_type) => {
                self.batting_team = team_type;
            }
            GameEvent::EndInnings => {
                self.batting_team = match self.batting_team {
                    TeamType::A => TeamType::B,
                    TeamType::B => TeamType::A,
                };

                self.batter_a = None;
                self.batter_b = None;
                self.on_strike_batter = PlayerType::A;
                self.bowler = None;
                self.last_bowler = None;
                self.last_last_bowler = None;
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
            let player = Rc::clone(player);
            let mut batting_container = Row::<Event>::new();
            if self.on_strike_batter == PlayerType::A {
                batting_container = batting_container.push(text("*"));
            }
            batting_container =
                batting_container.push(player.borrow().clone().to_batting_container());
            content = content.push(batting_container);
        }

        if let Some(player) = &self.batter_b {
            let player = Rc::clone(player);
            let mut batting_container = Row::<Event>::new();
            if self.on_strike_batter == PlayerType::B {
                batting_container = batting_container.push(text("*"));
            }
            batting_container =
                batting_container.push(player.borrow().clone().to_batting_container());
            content = content.push(batting_container);
        }

        content = content.push(team.extras.to_container());

        if let Some(player) = &self.bowler {
            let player = Rc::clone(player);
            content = content.push(player.borrow().clone().to_bowling_container());
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

        for event in &self.event_history {
            if let Some(event_container) = event.to_container() {
                column = column.push(event_container);
            }

            if let Some(event_container) = self.events[event.event_index].to_container() {
                column = column.push(event_container);
            }
        }

        column
    }

    pub fn player_column(&self) -> Column<Event> {
        let team = self.batting_team();
        let mut column = Column::new();

        for player in &team.players {
            column = column.push(player.borrow().clone().to_container());
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
            event_history: vec![],
            batting_team: TeamType::A,
            batter_a: None,
            batter_b: None,
            on_strike_batter: PlayerType::A,
            bowler: None,
            last_bowler: None,
            last_last_bowler: None,
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

    pub fn bowling_team(&self) -> &Team {
        let team = match self.batting_team {
            TeamType::A => &self.team_b,
            TeamType::B => &self.team_a,
        };

        team
    }

    fn add_event(&mut self, event: GameEvent) {
        let event_index = self.events.len();
        let batter = self.on_strike_batter();
        let bowler = if let Some(bowler) = &self.bowler {
            Some(Rc::clone(bowler))
        } else {
            None
        };
        self.events.push(event.clone());
        self.event_history
            .push(GameEventHistory::new(event_index, bowler, batter));
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
        let on_strike_batter = Rc::clone(
            &self
                .on_strike_batter()
                .expect("A player should be on strike when add_runs is callled"),
        );
        let mut on_strike_batter = on_strike_batter.borrow_mut();

        on_strike_batter.balls_faced += 1;
        on_strike_batter.runs_scored += runs;

        let bowler = Rc::clone(
            &self
                .bowler
                .as_ref()
                .expect("A player should be bowling when add_runs is callled"),
        );
        let mut bowler = bowler.borrow_mut();

        bowler.overs_bowled.add_ball_bowler();
        bowler.runs_conceded += runs;

        let team = self.batting_team_mut();
        team.runs += runs;
        team.overs.add_ball();

        if runs % 2 == 1 {
            self.change_strike();
        }
    }

    fn add_wicket(&mut self, wicket_event: &WicketEvent) {
        let player = Rc::clone(
            &self
                .on_strike_batter()
                .expect("There should be an on strike batter when a wicket occurs")
                .to_owned(),
        );
        let mut player = player.borrow_mut();
        player.how_out = wicket_event.how_out.clone();
        player.wicket_details = Some(WicketDetails::new(
            wicket_event.bowler,
            wicket_event.fielder,
        ));

        let team = self.batting_team_mut();
        team.wickets += 1;
        team.overs.add_ball();

        let bowler = Rc::clone(
            &self
                .bowler
                .as_ref()
                .expect("There should be a bowler when a wicket occurs"),
        );
        let mut bowler = bowler.borrow_mut();
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

    pub fn on_strike_batter(&self) -> Option<Rc<RefCell<Player>>> {
        let batter = match self.on_strike_batter {
            PlayerType::A => &self.batter_a,
            PlayerType::B => &self.batter_b,
        };

        let mut player_ref = None;

        if let Some(player) = batter {
            player_ref = Some(Rc::clone(player));
        }

        player_ref
    }

    pub fn set_on_strike_batter(&mut self, batter: Option<Rc<RefCell<Player>>>) {
        match self.on_strike_batter {
            PlayerType::A => self.batter_a = batter,
            PlayerType::B => self.batter_b = batter,
        }
    }

    fn is_end_over(&self) -> bool {
        self.batting_team().overs.balls >= 6
    }

    fn end_over(&mut self) {
        self.last_last_bowler = self.last_bowler;
        self.last_bowler = Some(
            self.bowler
                .as_ref()
                .expect("There should be a bowler")
                .borrow()
                .order,
        );
        self.bowler = None;

        self.batting_team_mut().overs.end_over();
        self.change_strike();
        self.add_event(GameEvent::EndOver);
    }
}

pub enum ReplaceBatter {
    OnStrike,
    OffStrike,
}
