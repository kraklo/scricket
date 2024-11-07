mod game_state;

use game_state::event::{AppEvent, Event, GameEvent};
use game_state::{GameState, TeamType};
use iced::widget::{button, column, row, text, text_input};
use iced::Element;

fn main() -> iced::Result {
    iced::run("scricket", State::update, State::view)
}

struct State {
    page: Page,
    game_state: GameState,
    first_name_input: String,
    last_name_input: String,
}

impl State {
    fn update(&mut self, event: Event) {
        match event {
            Event::AppEvent(event) => match event {
                AppEvent::FirstNameChanged(first_name) => self.first_name_input = first_name,
                AppEvent::LastNameChanged(last_name) => self.last_name_input = last_name,
                AppEvent::SubmitName => {
                    self.game_state
                        .add_player(&self.first_name_input, &self.last_name_input);
                    self.first_name_input.clear();
                    self.last_name_input.clear();
                }
                AppEvent::SubmitTeam => match self.game_state.current_team {
                    TeamType::A => self.game_state.change_team(),
                    TeamType::B => self.page = Page::Scoring,
                },
            },
            Event::GameEvent(event) => {
                self.game_state.update(event);
            }
        }
    }

    fn view(&self) -> Element<Event> {
        match self.page {
            Page::TeamEntry => self.enter_player(),
            Page::Scoring => self.game_state.view(),
        }
    }

    fn enter_player(&self) -> Element<Event> {
        let mut column = column![
            row![
                text_input("First Name", &self.first_name_input)
                    .on_input(|input| Event::AppEvent(AppEvent::FirstNameChanged(input))),
                text_input("Last Name", &self.last_name_input)
                    .on_input(|input| Event::AppEvent(AppEvent::LastNameChanged(input))),
                button("Submit").on_press(Event::AppEvent(AppEvent::SubmitName)),
            ],
            self.game_state.player_column(),
        ];

        if self.game_state.team_length() >= 11 {
            column =
                column.push(button("Confirm Team").on_press(Event::AppEvent(AppEvent::SubmitTeam)));
        }

        column.into()
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            page: Page::TeamEntry,
            game_state: GameState::new(),
            first_name_input: String::new(),
            last_name_input: String::new(),
        }
    }
}

enum Page {
    TeamEntry,
    Scoring,
}
