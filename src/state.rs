mod component;
mod event;
pub mod game_state;

use std::fs;

use bincode;
use component::Component;
use event::Event;
use game_state::event::GameEvent;
use game_state::GameState;
use iced::Element;
use rfd::FileDialog;

use component::batter_select::BatterSelect;
use component::bowler_select::BowlerSelect;
use component::start::Start;
use component::team_entry::TeamEntry;

pub struct State {
    game_state: GameState,
    page: Page,
    component: Box<dyn Component>,
}

impl State {
    // ui
    pub fn update(&mut self, event: Event) {
        match event {
            Event::ComponentEvent(component_event) => {
                let (game_state, page) = self
                    .component
                    .update(component_event, self.game_state.clone());

                if let Some(page) = page {
                    self.set_page(page);
                }
                self.game_state = game_state;
            }
            Event::GameEvent(game_event) => self.game_state.update(game_event),
            Event::LoadGame => self.load_game(),
            Event::SaveGame => self.save_game(),
        }
    }

    pub fn view(&self) -> Element<Event> {
        match self.page {
            Page::Scoring => self.game_state.view(),
            Page::Start => self.component.view(&self.game_state),
            Page::TeamEntry => self.component.view(&self.game_state),
            Page::SelectBatter => self.component.view(&self.game_state),
            Page::SelectBowler => self.component.view(&self.game_state),
        }
    }
}

impl State {
    fn load_game(&mut self) {
        let file_path = FileDialog::new().pick_file().expect("invalid path");
        let data = fs::read(file_path).expect("file reading error");
        let deserialized: Vec<GameEvent> =
            bincode::deserialize(data.as_slice()).expect("error deserialising");
        self.set_page(Page::Scoring);
        self.game_state = GameState::from_events(deserialized);
    }

    fn save_game(&self) {
        let file_path = FileDialog::new().save_file().expect("invalid path");
        let serialized = bincode::serialize(&self.game_state.events).expect("error serialising");
        fs::write(file_path, serialized.as_slice()).expect("error writing file");
    }

    fn set_page(&mut self, page: Page) {
        match page {
            Page::Start => self.component = Box::new(Start::new()),
            Page::SelectBatter => self.component = Box::new(BatterSelect::new()),
            Page::SelectBowler => self.component = Box::new(BowlerSelect::new()),
            Page::TeamEntry => self.component = Box::new(TeamEntry::new()),
            _ => (),
        }

        self.page = page;
    }
}

impl Default for State {
    fn default() -> Self {
        State {
            game_state: GameState::new(),
            page: Page::Start,
            component: Box::new(Start::new()),
        }
    }
}

#[derive(Clone)]
enum Page {
    Start,
    TeamEntry,
    Scoring,
    SelectBatter,
    SelectBowler,
}
