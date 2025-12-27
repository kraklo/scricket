pub mod event;
pub mod game_state;

use std::fs;

use bincode;
use event::Event;
use game_state::event::GameEvent;
use game_state::GameState;
use iced::Element;
use rfd::FileDialog;
use serde_json;

use crate::components::batter_select::BatterSelect;
use crate::components::bowler_select::BowlerSelect;
use crate::components::extra_select::ExtraSelect;
use crate::components::innings_select::InningsSelect;
use crate::components::start::Start;
use crate::components::team_entry::TeamEntry;
use crate::components::wicket_select::WicketSelect;
use crate::components::Component;

pub struct State {
    game_state: GameState,
    page: Page,
    component: Box<dyn Component>,
}

impl State {
    // ui
    pub fn update(&mut self, event: Event) {
        let mut page = None;

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
            Event::GameEvent(game_event) => page = self.game_state.update(game_event),
            Event::ChangePage(page) => self.set_page(page),
            Event::LoadGame => self.load_game(),
            Event::SaveGame => self.save_game(),
        }

        if let Some(page) = page {
            self.set_page(page);
        }
    }

    pub fn view(&self) -> Element<Event> {
        match self.page {
            Page::Scoring => self.game_state.view(),
            _ => self.component.view(&self.game_state),
        }
    }
}

impl State {
    fn load_game(&mut self) {
        let file_path = FileDialog::new()
            .add_filter("scricket", &["scr"])
            .add_filter("json", &["json"])
            .pick_file()
            .expect("invalid path");

        let extension = file_path.extension().unwrap().to_str().unwrap();
        let data = fs::read(&file_path).expect("file reading error");

        let deserialized: Vec<GameEvent> = if extension == "scr" {
            bincode::deserialize(data.as_slice()).expect("error deserialising")
        } else if extension == "json" {
            serde_json::from_slice(data.as_slice()).expect("error deserialising")
        } else {
            panic!("{} is not a valid extension", extension)
        };

        self.set_page(Page::Scoring);
        self.game_state = GameState::from_events(deserialized);
    }

    fn save_game(&self) {
        let file_path = FileDialog::new()
            .add_filter("scricket", &["scr"])
            .add_filter("json", &["json"])
            .save_file()
            .expect("invalid path");

        let extension = file_path.extension().unwrap().to_str().unwrap();

        let serialized = if extension == "scr" {
            bincode::serialize(&self.game_state.events).expect("error serialising")
        } else if extension == "json" {
            serde_json::to_string(&self.game_state.events)
                .expect("error serializing")
                .as_bytes()
                .to_vec()
        } else {
            panic!("{} is not a valid extension", extension)
        };

        fs::write(file_path, serialized.as_slice()).expect("error writing file");
    }

    fn set_page(&mut self, page: Page) {
        match page {
            Page::Start => self.component = Box::new(Start::new()),
            Page::SelectBatter => self.component = Box::new(BatterSelect::new()),
            Page::SelectBowler => self.component = Box::new(BowlerSelect::new(&self.game_state)),
            Page::TeamEntry => self.component = Box::new(TeamEntry::new()),
            Page::SelectWicket => self.component = Box::new(WicketSelect::new()),
            Page::SelectExtra => self.component = Box::new(ExtraSelect::new()),
            Page::SelectInnings => self.component = Box::new(InningsSelect::new()),
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

#[derive(Debug, Clone)]
pub enum Page {
    Start,
    TeamEntry,
    Scoring,
    SelectBatter,
    SelectBowler,
    SelectWicket,
    SelectExtra,
    SelectInnings,
}
