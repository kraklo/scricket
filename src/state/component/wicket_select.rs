use super::fielder_select::FielderSelect;
use super::runout_select::RunoutSelect;
use super::{AsEvent, Component, ComponentEvent, Subcomponent};
use crate::state::game_state::event::GameEvent;
use crate::state::game_state::wickets::{HowOut, WicketEvent};
use crate::state::game_state::PlayerType;
use crate::state::{Event, GameState, Page};
use iced::widget::{button, column, radio, row, text};
use iced::Element;
use macros::AsComponentEvent;
use strum::IntoEnumIterator;

pub enum WicketSubcomponentData {
    CaughtSubcomponent(usize),
    RunoutSubcomponent((usize, Option<usize>)),
}

pub struct WicketSelect {
    selected_how_out: Option<usize>,
    subcomponent: Option<Box<dyn Subcomponent<WicketSubcomponentData>>>,
}

impl Component for WicketSelect {
    fn update(
        &mut self,
        event: ComponentEvent,
        mut game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let mut page = None;

        let event = match event {
            ComponentEvent::WicketSelectEvent(wicket_select_event) => wicket_select_event,
            ComponentEvent::SubcomponentEvent(subcomponent_event) => {
                (game_state, page) = self
                    .subcomponent
                    .as_mut()
                    .expect("extra component should exist")
                    .update(subcomponent_event.clone(), game_state);

                WicketSelectEvent::SubcomponentEvent
            }
            _ => {
                panic!("Wicket component has been called with an event that is not a wicket event!")
            }
        };

        match event {
            WicketSelectEvent::HowOutSelected(how_out_index) => {
                self.selected_how_out = Some(how_out_index);
                let how_out = HowOut::iter().collect::<Vec<HowOut>>()[self
                    .selected_how_out
                    .expect("How out should be selected when a wicket is submitted")]
                .clone();
                let players = &game_state.bowling_team().players;

                match how_out {
                    HowOut::Caught => {
                        self.subcomponent = Some(Box::new(FielderSelect::new(players.to_owned())))
                    }
                    HowOut::RunOut => {
                        self.subcomponent = Some(Box::new(RunoutSelect::new(&game_state)))
                    }
                    _ => self.subcomponent = None,
                }
            }
            WicketSelectEvent::SubmitWicket => {
                let how_out = HowOut::iter().collect::<Vec<HowOut>>()[self
                    .selected_how_out
                    .expect("How out should be selected when a wicket is submitted")]
                .clone();
                let bowler = Some(game_state.bowler.as_ref().unwrap().borrow().order);
                let data = match &self.subcomponent {
                    Some(subcomponent) => subcomponent.get_value(),
                    None => None,
                };

                let fielder: Option<usize>;

                match data {
                    Some(data) => match data {
                        WicketSubcomponentData::CaughtSubcomponent(fielder_num) => {
                            fielder = Some(fielder_num)
                        }
                        WicketSubcomponentData::RunoutSubcomponent((
                            selected_batter,
                            fielder_num,
                        )) => {
                            let batter_num = match game_state.on_strike_batter {
                                PlayerType::A => 0,
                                PlayerType::B => 1,
                            };

                            if batter_num != selected_batter {
                                game_state.change_strike();
                            }

                            fielder = fielder_num;
                        }
                    },
                    None => fielder = None,
                }

                game_state.update(GameEvent::Wicket(WicketEvent::new(
                    how_out, bowler, fielder,
                )));
                page = Some(Page::SelectBatter);
            }
            WicketSelectEvent::SubcomponentEvent => (),
        }

        (game_state, page)
    }

    fn view<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event> {
        let mut column = column![text("Select how out:")];

        for (i, how_out) in HowOut::iter().enumerate() {
            match how_out {
                HowOut::DidNotBat | HowOut::NotOut => continue,
                _ => (),
            }
            column = column.push(radio(
                how_out.to_string(),
                i,
                self.selected_how_out,
                |selection| WicketSelectEvent::HowOutSelected(selection).as_event(),
            ));
        }

        if self.selected_how_out.is_some()
            && (self.subcomponent.is_none() || self.subcomponent.as_ref().unwrap().can_submit())
        {
            column = column.push(
                button("Select how out").on_press(WicketSelectEvent::SubmitWicket.as_event()),
            );
        }

        let mut row = row![column];

        if let Some(component) = &self.subcomponent {
            row = row.push(component.view(&game_state));
        }

        row.into()
    }
}

impl WicketSelect {
    pub fn new() -> Self {
        Self {
            selected_how_out: None,
            subcomponent: None,
        }
    }
}

#[derive(Clone, Debug, AsComponentEvent)]
pub enum WicketSelectEvent {
    HowOutSelected(usize),
    SubmitWicket,
    SubcomponentEvent,
}
