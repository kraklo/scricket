use super::Event;
use super::{Player, TeamType};
use crate::state::game_state::extras::Extra;
use crate::state::game_state::wickets::HowOut;
use iced::widget::{container, text, Container};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum GameEvent {
    Runs(u32),
    Extra(Extra),
    Wicket(HowOut),
    StartOver,
    EndOver,
    StartInnings(TeamType),
    EndInnings(TeamType),
    SelectOnStrike(usize),
    SelectOffStrike(usize),
    SelectBowler(usize),
    AddPlayer(Player),
    SubmitTeam,
}

impl GameEvent {
    pub fn to_container(&self) -> Option<Container<Event>> {
        let container_text = match self {
            Self::Runs(runs) => format!(
                "{runs} run{plural}",
                plural = if *runs == 1 { "" } else { "s" }
            ),
            Self::Wicket(how_out) => String::from(format!("wicket: {}", how_out.to_string())),
            Self::Extra(extra) => String::from(format!("extra: {}", extra.extra_type.to_string())),
            _ => return None,
        };

        Some(container(text(container_text)))
    }
}

#[derive(Debug, Clone)]
pub struct GameEventHistory {
    pub event_index: usize,
    pub bowler: Option<Rc<RefCell<Player>>>,
    pub batter: Option<Rc<RefCell<Player>>>,
}

impl GameEventHistory {
    pub fn new(
        event_index: usize,
        bowler: Option<Rc<RefCell<Player>>>,
        batter: Option<Rc<RefCell<Player>>>,
    ) -> Self {
        GameEventHistory {
            event_index,
            bowler,
            batter,
        }
    }

    pub fn to_container(&self) -> Option<Container<Event>> {
        if let Some(bowler) = &self.bowler {
            if let Some(batter) = &self.batter {
                return Some(container(text(format!(
                    "{bowler} to {batter}",
                    bowler = bowler.borrow(),
                    batter = batter.borrow()
                ))));
            }
        }

        None
    }
}
