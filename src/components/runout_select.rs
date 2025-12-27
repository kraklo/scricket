use crate::components::fielder_select::{FielderSelect, FielderSelectEvent};
use crate::components::wicket_select::WicketSubcomponentData;
use crate::components::{AsEvent, ComponentEvent, Subcomponent, SubcomponentEvent};
use crate::state::event::Event;
use crate::state::game_state::GameState;
use crate::state::Page;
use macros::AsSubcomponentEvent;

use iced::widget::row;
use iced::Element;

pub struct RunoutSelect {
    batter_select: FielderSelect,
    fielder_select: FielderSelect,
}

impl Subcomponent<WicketSubcomponentData> for RunoutSelect {
    fn update(
        &mut self,
        event: SubcomponentEvent,
        game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            SubcomponentEvent::RunoutSelectEvent(runout_select_event) => runout_select_event,
            _ => panic!("Runout Select component has been called with an event that is not a runout select event!")
        };

        match event {
            RunoutSelectEvent::BatterSelectEvent(selection) => {
                self.batter_select.update(
                    SubcomponentEvent::FielderSelectEvent(FielderSelectEvent::FielderSelected(
                        selection,
                    )),
                    GameState::new(),
                );
            }
            RunoutSelectEvent::FielderSelectEvent(selection) => {
                self.fielder_select.update(
                    SubcomponentEvent::FielderSelectEvent(FielderSelectEvent::FielderSelected(
                        selection,
                    )),
                    GameState::new(),
                );
            }
        }

        (game_state, None)
    }

    fn view<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event> {
        row![
            self.batter_select.view(&game_state),
            self.fielder_select.view(&game_state),
        ]
        .into()
    }

    fn can_submit(&self) -> bool {
        true
    }

    fn get_value(&self) -> Option<WicketSubcomponentData> {
        if let WicketSubcomponentData::CaughtSubcomponent(batter) =
            self.batter_select.get_value().unwrap()
        {
            let fielder = match self.fielder_select.get_value() {
                Some(data) => match data {
                    WicketSubcomponentData::CaughtSubcomponent(fielder) => Some(fielder),
                    _ => panic!("wrong fielder data"),
                },
                None => None,
            };

            return Some(WicketSubcomponentData::RunoutSubcomponent((
                batter, fielder,
            )));
        }

        panic!("wrong batter data");
    }
}

impl RunoutSelect {
    pub fn new(game_state: &GameState) -> Self {
        let batters = vec![
            game_state.batter_a.as_ref().unwrap().clone(),
            game_state.batter_b.as_ref().unwrap().clone(),
        ];
        let fielders = game_state.bowling_team().players.clone();

        let mut component = Self {
            batter_select: FielderSelect::new_with_selection_fn(
                batters,
                Box::new(|selection| RunoutSelectEvent::BatterSelectEvent(selection).as_event()),
            )
            .with_message(String::from("Select run out batter:")),
            fielder_select: FielderSelect::new_with_selection_fn(
                fielders,
                Box::new(|selection| RunoutSelectEvent::FielderSelectEvent(selection).as_event()),
            ),
        };

        component.batter_select.select_player(0);
        component
    }
}

#[derive(Clone, Debug, AsSubcomponentEvent)]
pub enum RunoutSelectEvent {
    BatterSelectEvent(usize),
    FielderSelectEvent(usize),
}
