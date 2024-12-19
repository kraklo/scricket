use super::{AsEvent, Component, ComponentEvent};
use crate::state::game_state::event::GameEvent;
use crate::state::{Event, GameState, Page};
use iced::widget::{button, radio, text, Column};
use iced::Element;
use macros::AsEvent;

pub struct BowlerSelect {
    selected_player: Option<u32>,
}

impl Component for BowlerSelect {
    fn update(
        &mut self,
        event: ComponentEvent,
        mut game_state: GameState,
    ) -> (GameState, Option<Page>) {
        let event = match event {
            ComponentEvent::BowlerSelectEvent(bowler_select_event) => bowler_select_event,
            _ => panic!("Bowler Select component has been called with an event that is not a bowler select event!")
        };

        let mut page = None;

        match event {
            BowlerSelectEvent::BowlerSelected(order) => self.selected_player = Some(order),
            BowlerSelectEvent::SubmitBowler => {
                game_state.update(GameEvent::SelectBowler(
                    game_state
                        .bowling_team()
                        .player_from_order(self.selected_player.expect("Bowler should be selected"))
                        .expect("Selected player should exist"),
                ));
                page = Some(Page::Scoring);
            }
        }

        (game_state, page)
    }

    fn view<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event> {
        match game_state.bowler {
            None => self.select_bowler(game_state),
            Some(_) => panic!("There should be a bowler to select when on this page"),
        }
    }
}

impl BowlerSelect {
    pub fn new() -> Self {
        BowlerSelect {
            selected_player: None,
        }
    }

    fn select_bowler<'a>(&self, game_state: &'a GameState) -> Element<'a, Event> {
        let team = game_state.bowling_team();
        let mut column = Column::new();
        column = column.push(text("Select bowler"));

        for player in &team.players {
            if let Some(player) = player {
                column = column.push(radio(
                    player.to_string(),
                    player.order,
                    self.selected_player,
                    |selection| BowlerSelectEvent::BowlerSelected(selection).as_event(),
                ));
            };
        }

        if let Some(_) = self.selected_player {
            column = column
                .push(button("Select player").on_press(BowlerSelectEvent::SubmitBowler.as_event()));
        }

        column.into()
    }
}

#[derive(Clone, Debug, AsEvent)]
pub enum BowlerSelectEvent {
    BowlerSelected(u32),
    SubmitBowler,
}
