use super::{AsEvent, Component, ComponentEvent};
use crate::state::game_state::event::GameEvent;
use crate::state::game_state::team::player::Player;
use crate::state::{Event, GameState, Page};
use iced::widget::{button, radio, text, Column, Radio};
use iced::Element;
use macros::AsComponentEvent;
use std::cell::RefCell;
use std::rc::Rc;

pub struct BowlerSelect {
    selected_player: Option<usize>,
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
                    self.selected_player.expect("Selected player should exist"),
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
    pub fn new(game_state: &GameState) -> Self {
        BowlerSelect {
            selected_player: game_state.last_last_bowler,
        }
    }

    fn select_bowler<'a>(&'a self, game_state: &'a GameState) -> Element<'a, Event> {
        let team = game_state.bowling_team();
        let mut column = Column::new();
        column = column.push(text("Select bowler"));

        let bowled_players = team.bowled_players_in_order();

        if bowled_players.len() > 0 {
            for player in bowled_players {
                column = column.push(self.display_bowler(&player));
            }
        }

        let not_bowled_players = team.not_bowled_players();

        if not_bowled_players.len() > 0 {
            column = column.push("Players that haven't bowled yet:");
            for player in not_bowled_players {
                column = column.push(self.display_bowler(&player));
            }
        }

        if let Some(_) = self.selected_player {
            column = column
                .push(button("Select player").on_press(BowlerSelectEvent::SubmitBowler.as_event()));
        }

        column.into()
    }

    fn display_bowler(&self, player: &Rc<RefCell<Player>>) -> Radio<Event> {
        let player = player.borrow();
        let mut label = player.to_string();

        if player.bowling_order != None {
            label += &format!(
                " {}/{} ({}.{})",
                player.wickets_taken,
                player.runs_conceded,
                player.overs_bowled.overs,
                player.overs_bowled.balls,
            );
        }

        radio(label, player.order, self.selected_player, |selection| {
            BowlerSelectEvent::BowlerSelected(selection).as_event()
        })
    }
}

#[derive(Clone, Debug, AsComponentEvent)]
pub enum BowlerSelectEvent {
    BowlerSelected(usize),
    SubmitBowler,
}
