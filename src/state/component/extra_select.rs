use super::{AsEvent, Component, ComponentEvent};
use crate::state::game_state::event::GameEvent;
use crate::state::game_state::event::{Extra, ExtraType};
use crate::state::{Event, GameState, Page};
use iced::widget::{button, column, radio, text};
use iced::Element;
use macros::AsEvent;
use strum::IntoEnumIterator;

pub struct ExtraSelect {
    selected_extra: Option<usize>,
}

impl Component for ExtraSelect {
    fn update(
        &mut self,
        event: ComponentEvent,
        mut game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            ComponentEvent::ExtraSelectEvent(extra_select_event) => extra_select_event,
            _ => panic!("Extra select component has been called with an event that is not a extra select event!")
        };

        let mut page = None;

        match event {
            ExtraSelectEvent::ExtraSelected(extra_index) => self.selected_extra = Some(extra_index),
            ExtraSelectEvent::SubmitExtra => {
                let extra_type = ExtraType::iter().collect::<Vec<ExtraType>>()[self
                    .selected_extra
                    .expect("Extra should be selected when an extra is submitted")]
                .clone();

                let extra = Extra::new(extra_type);

                game_state.update(GameEvent::Extra(extra));
                page = Some(Page::Scoring);
            }
        }

        (game_state, page)
    }

    fn view<'a>(&'a self, _: &'a GameState) -> Element<'a, Event> {
        let mut column = column![text("Select extra:")];

        for (i, extra) in ExtraType::iter().enumerate() {
            column = column.push(radio(
                extra.to_string(),
                i,
                self.selected_extra,
                |selection| ExtraSelectEvent::ExtraSelected(selection).as_event(),
            ));
        }

        if let Some(_) = self.selected_extra {
            column = column
                .push(button("Select extra").on_press(ExtraSelectEvent::SubmitExtra.as_event()));
        }

        column.into()
    }
}

impl ExtraSelect {
    pub fn new() -> Self {
        ExtraSelect {
            selected_extra: None,
        }
    }
}

#[derive(Clone, Debug, AsEvent)]
pub enum ExtraSelectEvent {
    ExtraSelected(usize),
    SubmitExtra,
}
