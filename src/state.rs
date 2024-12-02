pub mod app_state;
pub mod game_state;

use std::fs;

use app_state::{AppState, Page};
use bincode;
use game_state::event::{Event, GameEvent};
use game_state::GameState;
use iced::Element;
use rfd::FileDialog;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
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
            Event::LoadGame => self.load_game(),
            Event::SaveGame => self.save_game(),
        }
    }

    pub fn view(&self) -> Element<Event> {
        match self.app_state.page {
            Page::Scoring => self.game_state.view(),
            _ => self.app_state.view(&self.game_state),
        }
    }
}

impl State {
    fn load_game(&mut self) {
        let file_path = FileDialog::new().pick_file().expect("invalid path");
        let data = fs::read(file_path).expect("file reading error");
        let deserialized: State =
            bincode::deserialize(data.as_slice()).expect("error deserialising");
        self.app_state = deserialized.app_state;
        self.game_state = deserialized.game_state;
    }

    fn save_game(&self) {
        let file_path = FileDialog::new().save_file().expect("invalid path");
        let serialized = bincode::serialize(self).expect("error serialising");
        fs::write(file_path, serialized.as_slice()).expect("error writing file");
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
