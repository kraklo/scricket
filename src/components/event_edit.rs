use macros::AsComponentEvent;

use iced::widget::pick_list;

use crate::components::{AsEvent, ComponentEvent};
use crate::state::event::Event;

#[derive(Clone, Debug)]
pub struct EventEdit {
    event_index: usize,
}

impl EventEdit {
    pub fn new(event_index: usize) -> Self {
        Self { event_index }
    }

    pub fn to_element<'a>(&'a self) -> iced::Element<'a, crate::state::event::Event> {
        let edit_options = [EventEditEvent::EventDeleted(self.event_index)];

        pick_list(edit_options, None::<EventEditEvent>, |_| {
            Event::DeleteEvent(self.event_index)
        })
        .placeholder("...")
        .into()
    }
}

#[derive(Clone, Debug, AsComponentEvent, PartialEq)]
pub enum EventEditEvent {
    EventDeleted(usize),
}

impl std::fmt::Display for EventEditEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            EventEditEvent::EventDeleted(_) => "Delete",
        })
    }
}
