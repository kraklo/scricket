use super::{AsEvent, Component, ComponentEvent};
use crate::state::game_state::event::GameEvent;
use crate::state::game_state::wickets::HowOut;
use crate::state::game_state::ReplaceBatter;
use crate::state::{Event, GameState, Page};
use iced::widget::{button, column, radio, text, Column};
use iced::Element;
use macros::AsComponentEvent;

pub struct BatterSelect {
    selected_player: Option<usize>,
}

impl Component for BatterSelect {
    fn update(
        &mut self,
        event: ComponentEvent,
        mut game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            ComponentEvent::BatterSelectEvent(batter_select_event) => batter_select_event,
            _ => panic!("Batter Select component has been called with an event that is not a batter select event!")
        };

        let mut page = None;

        match event {
            BatterSelectEvent::BatterSelected(order) => self.selected_player = Some(order),
            BatterSelectEvent::SubmitBatter => {
                if let Some(batter) = game_state.batter_to_replace() {
                    match batter {
                        ReplaceBatter::OnStrike => {
                            game_state.update(GameEvent::SelectOnStrike(
                                self.selected_player.expect("Selected player should exist"),
                            ));
                        }
                        ReplaceBatter::OffStrike => {
                            game_state.update(GameEvent::SelectOffStrike(
                                self.selected_player.expect("Selected player should exist"),
                            ));
                        }
                    }

                    self.selected_player = None;
                }

                if let None = game_state.batter_to_replace() {
                    page = match game_state.bowler {
                        Some(_) => Some(Page::Scoring),
                        None => Some(Page::SelectBowler),
                    }
                }
            }
        }

        (game_state, page)
    }

    fn view<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event> {
        match game_state.batter_to_replace() {
            Some(batter) => match batter {
                ReplaceBatter::OnStrike => self.select_on_strike_batter(game_state),
                ReplaceBatter::OffStrike => self.select_off_strike_batter(game_state),
            },
            None => panic!("There should be a batter to select when on this page"),
        }
    }
}

impl BatterSelect {
    pub fn new() -> Self {
        BatterSelect {
            selected_player: None,
        }
    }

    fn select_batter<'a>(&self, game_state: &'a GameState) -> Element<'a, Event> {
        let team = game_state.batting_team();
        let mut column = Column::new();

        for player in &team.players {
            let player = player.borrow();
            if (player.how_out != HowOut::DidNotBat
                && player.how_out != HowOut::RetiredHurt
                && player.how_out != HowOut::RetiredNotOut)
                || player.how_out == HowOut::NotOut
            {
                continue;
            }

            column = column.push(radio(
                player.to_string(),
                player.order,
                self.selected_player,
                |selection| BatterSelectEvent::BatterSelected(selection).as_event(),
            ));
        }

        if let Some(_) = self.selected_player {
            column = column
                .push(button("Select player").on_press(BatterSelectEvent::SubmitBatter.as_event()));
        }

        column.into()
    }

    fn select_on_strike_batter<'a>(&self, game_state: &'a GameState) -> Element<'a, Event> {
        let mut column = column![text("Select on strike batter")];
        column = column.push(self.select_batter(game_state));
        column.into()
    }

    fn select_off_strike_batter<'a>(&self, game_state: &'a GameState) -> Element<'a, Event> {
        let mut column = column![text("Select off strike batter")];
        column = column.push(self.select_batter(game_state));
        column.into()
    }
}

#[derive(Clone, Debug, AsComponentEvent)]
pub enum BatterSelectEvent {
    BatterSelected(usize),
    SubmitBatter,
}
