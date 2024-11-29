pub mod event;

use crate::state::game_state::team::TeamType;
use crate::state::game_state::{GameState, ReplaceBatter};
use crate::state::Event;
use crate::state::GameEvent;
use event::AppEvent;
use iced::widget::{button, column, radio, row, text, text_input, Column};
use iced::Element;

#[derive(Clone)]
pub struct AppState {
    pub page: Page,
    first_name_input: String,
    last_name_input: String,
    selected_player: Option<u32>,
    order: u32,
}

impl AppState {
    pub fn update(&mut self, app_event: AppEvent, mut game_state: GameState) -> GameState {
        match app_event {
            AppEvent::LoadGame => todo!(),
            AppEvent::NewGame => self.page = Page::TeamEntry,
            AppEvent::FirstNameChanged(first_name) => self.first_name_input = first_name,
            AppEvent::LastNameChanged(last_name) => self.last_name_input = last_name,
            AppEvent::SubmitName => {
                game_state.add_player(&self.first_name_input, &self.last_name_input, self.order);
                self.first_name_input.clear();
                self.last_name_input.clear();
                self.order += 1;
            }
            AppEvent::SubmitTeam => {
                match game_state.batting_team {
                    TeamType::A => game_state.change_team(),
                    TeamType::B => {
                        game_state.change_team();
                        self.page = Page::SelectBatter;
                    }
                }
                self.order = 0;
            }
            AppEvent::BatterSelected(order) | AppEvent::BowlerSelected(order) => {
                self.selected_player = Some(order)
            }
            AppEvent::SubmitBatter => {
                if let Some(batter) = game_state.batter_to_replace() {
                    match batter {
                        ReplaceBatter::OnStrike => game_state.update(GameEvent::SelectOnStrike(
                            game_state
                                .batting_team()
                                .player_from_order(
                                    self.selected_player.expect("Batter should be selected"),
                                )
                                .expect("Selected player should exist"),
                        )),
                        ReplaceBatter::OffStrike => game_state.update(GameEvent::SelectOffStrike(
                            game_state
                                .batting_team()
                                .player_from_order(
                                    self.selected_player.expect("Batter should be selected"),
                                )
                                .expect("Selected player should exist"),
                        )),
                    }
                }

                if let None = game_state.batter_to_replace() {
                    self.page = match game_state.bowler {
                        Some(_) => Page::Scoring,
                        None => Page::SelectBowler,
                    }
                }
            }
            AppEvent::SubmitBowler => {
                game_state.update(GameEvent::SelectBowler(
                    game_state
                        .bowling_team()
                        .player_from_order(self.selected_player.expect("Batter should be selected"))
                        .expect("Selected player should exist"),
                ));
                self.page = Page::Scoring;
            }
        }

        game_state
    }

    pub fn view<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event> {
        match self.page {
            Page::Start => self.start(),
            Page::TeamEntry => self.enter_player(game_state),
            Page::SelectBatter => match game_state.batter_to_replace() {
                Some(batter) => match batter {
                    ReplaceBatter::OnStrike => self.select_on_strike_batter(game_state),
                    ReplaceBatter::OffStrike => self.select_off_strike_batter(game_state),
                },
                None => panic!("There should be a batter to select when on this page"),
            },
            Page::SelectBowler => match game_state.bowler {
                None => self.select_bowler(game_state),
                Some(_) => panic!("There should be a bowler to select when on this page"),
            },
            _ => panic!("app_state shouldn't be expected to view this page!"),
        }
    }

    fn start(&self) -> Element<Event> {
        row![
            button("Load Game").on_press(Event::AppEvent(AppEvent::LoadGame)),
            button("New Game").on_press(Event::AppEvent(AppEvent::NewGame)),
        ]
        .into()
    }

    fn enter_player<'a>(&self, game_state: &'a GameState) -> Element<'a, Event> {
        let mut column = column![
            row![
                text_input("First Name", &self.first_name_input)
                    .on_input(|input| Event::AppEvent(AppEvent::FirstNameChanged(input))),
                text_input("Last Name", &self.last_name_input)
                    .on_input(|input| Event::AppEvent(AppEvent::LastNameChanged(input))),
                button("Submit").on_press(Event::AppEvent(AppEvent::SubmitName)),
            ],
            game_state.player_column(),
        ];

        if game_state.team_length() >= 11 {
            column =
                column.push(button("Confirm Team").on_press(Event::AppEvent(AppEvent::SubmitTeam)));
        }

        column.into()
    }

    fn select_batter<'a>(&self, game_state: &'a GameState) -> Element<'a, Event> {
        let team = game_state.batting_team();
        let mut column = Column::new();

        for player in &team.players {
            if let Some(player) = player {
                column = column.push(radio(
                    player.to_string(),
                    player.batting_order,
                    self.selected_player,
                    |selection| Event::AppEvent(AppEvent::BatterSelected(selection)),
                ));
            };
        }

        if let Some(_) = self.selected_player {
            column = column
                .push(button("Select player").on_press(Event::AppEvent(AppEvent::SubmitBatter)));
        }

        column.into()
    }

    fn select_bowler<'a>(&self, game_state: &'a GameState) -> Element<'a, Event> {
        let team = game_state.bowling_team();
        let mut column = Column::new();
        column = column.push(text("Select bowler"));

        for player in &team.players {
            if let Some(player) = player {
                column = column.push(radio(
                    player.to_string(),
                    player.batting_order,
                    self.selected_player,
                    |selection| Event::AppEvent(AppEvent::BowlerSelected(selection)),
                ));
            };
        }

        if let Some(_) = self.selected_player {
            column = column
                .push(button("Select player").on_press(Event::AppEvent(AppEvent::SubmitBowler)));
        }

        column.into()
    }

    fn select_on_strike_batter<'a>(&self, game_state: &'a GameState) -> Element<'a, Event> {
        let mut column = column![text("Select on strike batter")];
        column = column.push(self.select_batter(game_state));
        column.into()
    }

    fn select_off_strike_batter<'a>(&self, game_state: &'a GameState) -> Element<'a, Event> {
        let mut column = column![text("Select off strike batter")];
        column = column.push(self.select_batter(game_state));
        column.into()
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            page: Page::Start,
            first_name_input: String::new(),
            last_name_input: String::new(),
            selected_player: None,
            order: 0,
        }
    }
}

#[derive(Clone)]
pub enum Page {
    Start,
    TeamEntry,
    Scoring,
    SelectBatter,
    SelectBowler,
}
