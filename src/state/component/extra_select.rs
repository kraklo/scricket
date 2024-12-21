use super::runs_button::RunsButton;
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
    runs_button: Option<RunsButton>,
    runs_button_runs: u32,
}

impl Component for ExtraSelect {
    fn update(
        &mut self,
        event: ComponentEvent,
        mut game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            ComponentEvent::ExtraSelectEvent(extra_select_event) => extra_select_event,
            ComponentEvent::RunsButtonEvent(runs_button_event) => {
                let runs_button = self.runs_button.as_mut().expect("Runs button event in extra select with no runs button!");
                runs_button.update(runs_button_event);
                self.runs_button_runs = runs_button.runs;
                return (game_state, None);
            }
            _ => panic!("Extra select component has been called with an event that is not a extra select event!")
        };

        let mut page = None;

        match event {
            ExtraSelectEvent::ExtraSelected(extra_index) => {
                self.selected_extra = Some(extra_index);
                let extra_type = &ExtraType::iter().collect::<Vec<ExtraType>>()[self
                    .selected_extra
                    .expect("Extra should be selected when an extra is submitted")];
                let minimum_runs = match extra_type {
                    ExtraType::Wide | ExtraType::NoBall => 0,
                    _ => 1,
                };
                self.runs_button = Some(RunsButton::new(minimum_runs));
            }
            ExtraSelectEvent::SubmitExtra => {
                let extra_type = ExtraType::iter().collect::<Vec<ExtraType>>()[self
                    .selected_extra
                    .expect("Extra should be selected when an extra is submitted")]
                .clone();

                let extra = Extra::new(self.runs_button_runs, extra_type);

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
            column = column.push(
                self.runs_button
                    .as_ref()
                    .expect("Runs button should exist if extra is selected")
                    .view(),
            );
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
            runs_button: None,
            runs_button_runs: 0,
        }
    }
}

#[derive(Clone, Debug, AsEvent)]
pub enum ExtraSelectEvent {
    ExtraSelected(usize),
    SubmitExtra,
}
