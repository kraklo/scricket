pub mod batter_select;
pub mod bowler_select;
pub mod start;
pub mod team_entry;
pub mod wicket_select;

use crate::state::{Event, GameState, Page};
use batter_select::BatterSelectEvent;
use bowler_select::BowlerSelectEvent;
use iced::Element;
use start::StartEvent;
use team_entry::TeamEntryEvent;
use wicket_select::WicketSelectEvent;

pub trait Component {
    fn update(&mut self, event: ComponentEvent, game_state: GameState)
        -> (GameState, Option<Page>);
    fn view<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event>;
}

#[derive(Clone, Debug)]
pub enum ComponentEvent {
    BatterSelectEvent(BatterSelectEvent),
    BowlerSelectEvent(BowlerSelectEvent),
    StartEvent(StartEvent),
    TeamEntryEvent(TeamEntryEvent),
    WicketSelectEvent(WicketSelectEvent),
}

impl ComponentEvent {
    pub fn as_event(self) -> Event {
        Event::ComponentEvent(self)
    }
}
