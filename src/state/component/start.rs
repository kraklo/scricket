use super::{Component, ComponentEvent};
use crate::state::{Event, GameState, Page};
use iced::widget::{button, row};
use iced::Element;

pub struct Start {}

impl Component for Start {
    fn update(
        &mut self,
        event: ComponentEvent,
        game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            ComponentEvent::StartEvent(start_event) => start_event,
            _ => panic!("Batter Select component has been called with an event that is not a batter select event!")
        };

        let page;

        match event {
            StartEvent::NewGame => page = Some(Page::TeamEntry),
        }

        (game_state, page)
    }

    fn view<'a>(&'a self, _: &'a GameState) -> Element<'a, Event> {
        self.start()
    }
}

impl Start {
    pub fn new() -> Self {
        Start {}
    }

    fn start(&self) -> Element<Event> {
        row![
            button("Load Game").on_press(Event::LoadGame),
            button("New Game").on_press(ComponentEvent::StartEvent(StartEvent::NewGame).as_event()),
        ]
        .into()
    }
}

#[derive(Clone, Debug)]
pub enum StartEvent {
    NewGame,
}
