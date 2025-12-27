use crate::components::{AsEvent, Component, ComponentEvent};
use crate::state::game_state::event::GameEvent;
use crate::state::game_state::TeamType;
use crate::state::event::Event;
use crate::state::game_state::GameState;
use crate::state::Page;
use iced::widget::{button, column, radio, text};
use iced::Element;
use macros::AsComponentEvent;

pub struct InningsSelect {
    selected_team: Option<usize>,
}

impl Component for InningsSelect {
    fn update(
        &mut self,
        event: ComponentEvent,
        mut game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            ComponentEvent::InningsSelectEvent(innings_select_event) => innings_select_event,
            _ => panic!("Innings Select component has been called with an event that is not a innings select event!")
        };

        let mut page = None;

        match event {
            InningsSelectEvent::InningsSelected(order) => self.selected_team = Some(order),
            InningsSelectEvent::SubmitInnings => {
                let selected_team = self
                    .selection_to_team_type()
                    .expect("Should be a selected team when submitting");

                game_state.update(GameEvent::StartInnings(selected_team));
                page = Some(Page::SelectBatter);
            }
        }

        (game_state, page)
    }

    fn view<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event> {
        let mut column = column![
            text("Select first innings:"),
            radio(
                &game_state.team_a.team_name,
                0,
                self.selected_team,
                |selection| InningsSelectEvent::InningsSelected(selection).as_event(),
            ),
            radio(
                &game_state.team_b.team_name,
                1,
                self.selected_team,
                |selection| InningsSelectEvent::InningsSelected(selection).as_event(),
            ),
        ];

        if self.selected_team != None {
            column = column.push(
                button("Start innings").on_press(InningsSelectEvent::SubmitInnings.as_event()),
            )
        }

        column.into()
    }
}

impl InningsSelect {
    pub fn new() -> Self {
        Self {
            selected_team: None,
        }
    }

    fn selection_to_team_type(&self) -> Option<TeamType> {
        if self.selected_team? == 0 {
            Some(TeamType::A)
        } else if self.selected_team? == 1 {
            Some(TeamType::B)
        } else {
            None
        }
    }
}

#[derive(Clone, Debug, AsComponentEvent)]
pub enum InningsSelectEvent {
    InningsSelected(usize),
    SubmitInnings,
}
