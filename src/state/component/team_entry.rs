use super::{Component, ComponentEvent};
use crate::state::game_state::event::GameEvent;
use crate::state::game_state::team::player::Player;
use crate::state::game_state::TeamType;
use crate::state::{Event, GameState, Page};
use iced::widget::{button, column, row, text_input};
use iced::Element;

pub struct TeamEntry {
    first_name_input: String,
    last_name_input: String,
    order: u32,
}

impl Component for TeamEntry {
    fn update(
        &mut self,
        event: ComponentEvent,
        mut game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            ComponentEvent::TeamEntryEvent(team_entry_event) => team_entry_event,
            _ => panic!("Team Entry has been called with an event that is not a team entry event!"),
        };

        let mut page = None;

        match event {
            TeamEntryEvent::FirstNameChanged(first_name) => self.first_name_input = first_name,
            TeamEntryEvent::LastNameChanged(last_name) => self.last_name_input = last_name,
            TeamEntryEvent::SubmitName => {
                let player = Player::new(&self.first_name_input, &self.last_name_input, self.order);
                game_state.update(GameEvent::AddPlayer(player));
                self.first_name_input.clear();
                self.last_name_input.clear();
                self.order += 1;
            }
            TeamEntryEvent::SubmitTeam => {
                if game_state.batting_team == TeamType::B {
                    page = Some(Page::SelectBatter);
                }

                game_state.update(GameEvent::SubmitTeam);
                self.order = 0;
            }
        }

        (game_state, page)
    }

    fn view<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event> {
        let mut column = column![
            row![
                text_input("First Name", &self.first_name_input).on_input(|input| {
                    ComponentEvent::TeamEntryEvent(TeamEntryEvent::FirstNameChanged(input))
                        .as_event()
                }),
                text_input("Last Name", &self.last_name_input).on_input(|input| {
                    ComponentEvent::TeamEntryEvent(TeamEntryEvent::LastNameChanged(input))
                        .as_event()
                }),
                button("Submit").on_press(
                    ComponentEvent::TeamEntryEvent(TeamEntryEvent::SubmitName).as_event()
                ),
            ],
            game_state.player_column(),
        ];

        if game_state.team_length() >= 11 {
            column =
                column.push(button("Confirm Team").on_press(
                    ComponentEvent::TeamEntryEvent(TeamEntryEvent::SubmitTeam).as_event(),
                ));
        }

        column.into()
    }
}

impl TeamEntry {
    pub fn new() -> Self {
        TeamEntry {
            first_name_input: String::new(),
            last_name_input: String::new(),
            order: 0,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TeamEntryEvent {
    FirstNameChanged(String),
    LastNameChanged(String),
    SubmitName,
    SubmitTeam,
}
