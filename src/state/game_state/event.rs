use crate::components::event_edit::EventEdit;
use crate::state::game_state::extras::Extra;
use crate::state::game_state::summary::Summary;
use crate::state::game_state::wickets::WicketEvent;
use crate::state::game_state::Event;
use crate::state::game_state::{Player, TeamType};
use iced::widget::{container, row, text, Container};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GameEvent {
    Runs(u32),
    Extra(Extra),
    Wicket(WicketEvent),
    StartOver,
    EndOver(Summary),
    StartInnings(TeamType),
    EndInnings,
    SelectOnStrike(usize),
    SelectOffStrike(usize),
    SelectBowler(usize),
    AddPlayer(Player),
    SubmitTeam(String),
}

impl GameEvent {
    pub fn to_container(&self) -> Option<Container<Event>> {
        let container_text = match self {
            Self::Runs(runs) => format!(
                "{runs} run{plural}",
                plural = if *runs == 1 { "" } else { "s" }
            ),
            Self::Wicket(wicket_event) => {
                String::from(format!("wicket: {}", wicket_event.how_out.to_string()))
            }
            Self::Extra(extra) => String::from(format!("extra: {}", extra.extra_type.to_string())),
            Self::EndOver(summary) => String::from(format!(
                "End of over: {}/{} ({})",
                summary.wickets,
                summary.runs,
                summary.overs.to_string()
            )),
            _ => return None,
        };

        Some(container(text(container_text)))
    }

    pub fn is_ball(&self) -> bool {
        match *self {
            GameEvent::Runs(_) | GameEvent::Wicket(_) | GameEvent::Extra(_) => true,
            _ => false,
        }
    }

    pub fn is_setup_event(&self) -> bool {
        match *self {
            GameEvent::AddPlayer(_) | GameEvent::SubmitTeam(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct GameEventHistory {
    pub event_index: usize,
    pub bowler: Rc<RefCell<Player>>,
    pub batter: Rc<RefCell<Player>>,
    edit_widget: EventEdit,
}

impl GameEventHistory {
    pub fn new(
        event_index: usize,
        bowler: Rc<RefCell<Player>>,
        batter: Rc<RefCell<Player>>,
    ) -> Self {
        GameEventHistory {
            event_index,
            bowler,
            batter,
            edit_widget: EventEdit::new(event_index),
        }
    }

    pub fn to_element<'a>(&'a self) -> iced::Element<'a, Event> {
        return row![
            text(format!(
                "{bowler} to {batter}",
                bowler = self.bowler.borrow(),
                batter = self.batter.borrow()
            )),
            self.edit_widget.to_element()
        ]
        .into();
    }
}
