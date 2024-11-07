pub mod game_state;

use game_state::event::{AppEvent, Event};
use game_state::{GameState, TeamType};
use iced::widget::{button, column, row, text_input};
use iced::Element;

pub struct State {
    page: Page,
    game_state: GameState,
    first_name_input: String,
    last_name_input: String,
}

impl State {
    // ui
    pub fn update(&mut self, event: Event) {
        match event {
            Event::AppEvent(app_event) => self.handle_app_event(app_event),
            Event::GameEvent(game_event) => self.game_state.update(game_event),
        }
    }

    pub fn view(&self) -> Element<Event> {
        match self.page {
            Page::Start => self.start(),
            Page::TeamEntry => self.enter_player(),
            Page::Scoring => self.game_state.view(),
        }
    }

    fn start(&self) -> Element<Event> {
        row![
            button("Load Game").on_press(Event::AppEvent(AppEvent::LoadGame)),
            button("New Game").on_press(Event::AppEvent(AppEvent::NewGame)),
        ]
        .into()
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

impl State {
    // business logic
    fn handle_app_event(&mut self, app_event: AppEvent) {
        match app_event {
            AppEvent::LoadGame => todo!(),
            AppEvent::NewGame => self.page = Page::TeamEntry,
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
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            page: Page::Start,
            game_state: GameState::new(),
            first_name_input: String::new(),
            last_name_input: String::new(),
        }
    }
}

enum Page {
    Start,
    TeamEntry,
    Scoring,
}
