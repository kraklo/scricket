use super::{AsEvent, ComponentEvent, Subcomponent, SubcomponentEvent};
use crate::state::game_state::team::player::Player;
use crate::state::{Event, GameState, Page};

use std::cell::RefCell;
use std::rc::Rc;

use iced::widget::{column, radio, text};
use iced::Element;
use macros::AsSubcomponentEvent;

pub struct FielderSelect {
    players: Vec<Rc<RefCell<Player>>>,
    selected_player: Option<usize>,
}

impl Subcomponent<usize> for FielderSelect {
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
            FielderSelectEvent::FielderSelected(order) => self.selected_player = Some(order),
        }

        (game_state, None)
    }

    fn view<'a>(&'a self, _: &'a GameState) -> Element<'a, Event> {
        let mut column = column![text("Select fielder:")];

        for player in &self.players {
            let player = player.borrow();

            column = column.push(radio(
                player.to_string(),
                player.order,
                self.selected_player,
                |selection| FielderSelectEvent::FielderSelected(selection).as_event(),
            ));
        }

        column.into()
    }

    fn can_submit(&self) -> bool {
        self.selected_player != None
    }

    fn get_value(&self) -> Option<usize> {
        self.selected_player.clone()
    }
}

impl FielderSelect {
    pub fn new(players: Vec<Rc<RefCell<Player>>>) -> Self {
        Self {
            selected_player: None,
            players,
        }
    }
}

#[derive(Clone, Debug, AsSubcomponentEvent)]
pub enum FielderSelectEvent {
    FielderSelected(usize),
}
