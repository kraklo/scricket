use super::{AsEvent, Component, ComponentEvent};
use crate::state::game_state::event::GameEvent;
use crate::state::game_state::HowOut;
use crate::state::{Event, GameState, Page};
use iced::widget::{button, column, radio, text};
use iced::Element;
use macros::AsEvent;
use strum::IntoEnumIterator;

pub struct WicketSelect {
    selected_how_out: Option<usize>,
}

impl Component for WicketSelect {
    fn update(
        &mut self,
        event: ComponentEvent,
        mut game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            ComponentEvent::WicketSelectEvent(wicket_select_event) => wicket_select_event,
            _ => panic!("Batter Select component has been called with an event that is not a batter select event!")
        };

        let mut page = None;

        match event {
            WicketSelectEvent::HowOutSelected(how_out_index) => {
                self.selected_how_out = Some(how_out_index)
            }
            WicketSelectEvent::SubmitWicket => {
                let how_out = HowOut::iter().collect::<Vec<HowOut>>()[self
                    .selected_how_out
                    .expect("How out should be selected when a wicket is submitted")]
                .clone();
                game_state.update(GameEvent::Wicket(how_out));
                page = Some(Page::SelectBatter);
            }
        }

        (game_state, page)
    }

    fn view<'a>(&'a self, _: &'a GameState) -> Element<'a, Event> {
        let mut column = column![text("Select how out:")];

        for (i, how_out) in HowOut::iter().enumerate() {
            column = column.push(radio(
                how_out.to_string(),
                i,
                self.selected_how_out,
                |selection| WicketSelectEvent::HowOutSelected(selection).as_event(),
            ));
        }

        if let Some(_) = self.selected_how_out {
            column = column.push(
                button("Select how out").on_press(WicketSelectEvent::SubmitWicket.as_event()),
            );
        }

        column.into()
    }
}

impl WicketSelect {
    pub fn new() -> Self {
        WicketSelect {
            selected_how_out: None,
        }
    }
}

#[derive(Clone, Debug, AsEvent)]
pub enum WicketSelectEvent {
    HowOutSelected(usize),
    SubmitWicket,
}
