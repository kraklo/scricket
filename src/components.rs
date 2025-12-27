// components
pub mod batter_select;
pub mod bowler_select;
pub mod event_edit;
pub mod extra_select;
pub mod innings_select;
pub mod runs_button;
pub mod start;
pub mod team_entry;
pub mod wicket_select;

use batter_select::BatterSelectEvent;
use bowler_select::BowlerSelectEvent;
use extra_select::ExtraSelectEvent;
use innings_select::InningsSelectEvent;
use runs_button::RunsButtonEvent;
use start::StartEvent;
use team_entry::TeamEntryEvent;
use wicket_select::WicketSelectEvent;

// subcomponents
pub mod fielder_select;
pub mod runout_select;

use fielder_select::FielderSelectEvent;
use runout_select::RunoutSelectEvent;

use crate::components::event_edit::EventEditEvent;
use crate::state::event::Event;
use crate::state::game_state::GameState;
use crate::state::Page;
use iced::Element;

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
    ExtraSelectEvent(ExtraSelectEvent),
    RunsButtonEvent(RunsButtonEvent),
    InningsSelectEvent(InningsSelectEvent),
    SubcomponentEvent(SubcomponentEvent),
    EventEditEvent(EventEditEvent),
}

pub trait Subcomponent<T> {
    fn update(
        &mut self,
        event: SubcomponentEvent,
        game_state: GameState,
    ) -> (GameState, Option<Page>);
    fn view<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event>;
    fn can_submit(&self) -> bool;
    fn get_value(&self) -> Option<T>;
}

#[derive(Clone, Debug)]
pub enum SubcomponentEvent {
    FielderSelectEvent(FielderSelectEvent),
    RunoutSelectEvent(RunoutSelectEvent),
}

pub trait AsEvent {
    fn as_event(self) -> Event;
}
