pub mod batter_select;
pub mod bowler_select;
pub mod extra_select;
pub mod innings_select;
pub mod runs_button;
pub mod start;
pub mod team_entry;
pub mod wicket_select;

use crate::state::{Event, GameState, Page};
use batter_select::BatterSelectEvent;
use bowler_select::BowlerSelectEvent;
use extra_select::ExtraSelectEvent;
use iced::Element;
use innings_select::InningsSelectEvent;
use runs_button::RunsButtonEvent;
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
    ExtraSelectEvent(ExtraSelectEvent),
    RunsButtonEvent(RunsButtonEvent),
    InningsSelectEvent(InningsSelectEvent),
}

pub trait AsEvent {
    fn as_event(self) -> Event;
}
