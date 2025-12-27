use crate::components::wicket_select::WicketSubcomponentData;
use crate::components::{AsEvent, ComponentEvent, Subcomponent, SubcomponentEvent};
use crate::state::event::Event;
use crate::state::game_state::team::player::Player;
use crate::state::game_state::GameState;
use crate::state::Page;

use std::cell::RefCell;
use std::rc::Rc;

use iced::widget::{column, radio, text};
use iced::Element;
use macros::AsSubcomponentEvent;

pub struct FielderSelect {
    players: Vec<Rc<RefCell<Player>>>,
    selected_player: Option<usize>,
    selection_fn: Box<dyn Fn(usize) -> Event>,
    message: Option<String>,
}

impl Subcomponent<WicketSubcomponentData> for FielderSelect {
    fn update(
        &mut self,
        event: SubcomponentEvent,
        game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            SubcomponentEvent::FielderSelectEvent(fielder_select_event) => fielder_select_event,
            _ => panic!("Fielder Select component has been called with an event that is not a fielder select event!")
        };

        match event {
            FielderSelectEvent::FielderSelected(order) => self.select_player(order),
        }

        (game_state, None)
    }

    fn view<'a>(&'a self, _: &'a GameState) -> Element<'a, Event> {
        let mut column = column![text(
            self.message
                .clone()
                .unwrap_or(String::from("Select fielder:"))
        )];

        for player in &self.players {
            let player = player.borrow();

            column = column.push(radio(
                player.to_string(),
                player.order,
                self.selected_player,
                |selection| (self.selection_fn)(selection),
            ));
        }

        column.into()
    }

    fn can_submit(&self) -> bool {
        true
    }

    fn get_value(&self) -> Option<WicketSubcomponentData> {
        Some(WicketSubcomponentData::CaughtSubcomponent(
            self.selected_player?,
        ))
    }
}

impl FielderSelect {
    pub fn new(players: Vec<Rc<RefCell<Player>>>) -> Self {
        Self {
            selected_player: None,
            players,
            selection_fn: Box::new(|selection| {
                FielderSelectEvent::FielderSelected(selection).as_event()
            }),
            message: None,
        }
    }

    pub fn new_with_selection_fn(
        players: Vec<Rc<RefCell<Player>>>,
        selection_fn: Box<dyn Fn(usize) -> Event>,
    ) -> Self {
        Self {
            selected_player: None,
            players,
            selection_fn: selection_fn,
            message: None,
        }
    }

    pub fn select_player(&mut self, order: usize) {
        self.selected_player = Some(order);
    }

    pub fn with_message(mut self, message: String) -> Self {
        self.message = Some(message);
        self
    }
}

#[derive(Clone, Debug, AsSubcomponentEvent)]
pub enum FielderSelectEvent {
    FielderSelected(usize),
}
