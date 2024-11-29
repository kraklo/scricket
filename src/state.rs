pub mod app_state;
pub mod game_state;

use app_state::{AppState, Page};
use game_state::event::{Event, GameEvent};
use game_state::GameState;
use iced::Element;

#[derive(Clone)]
pub struct State {
    game_state: GameState,
    app_state: AppState,
}

impl State {
    // ui
    pub fn update(&mut self, event: Event) {
        match event {
            Event::AppEvent(app_event) => {
                self.game_state = self.app_state.update(app_event, self.game_state.clone())
            }
            Event::GameEvent(game_event) => self.game_state.update(game_event),
        }
    }

    pub fn view(&self) -> Element<Event> {
        match self.app_state.page {
            Page::Scoring => self.game_state.view(),
            _ => self.app_state.view(&self.game_state),
        }
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            game_state: GameState::new(),
            app_state: AppState::new(),
        }
    }
}
