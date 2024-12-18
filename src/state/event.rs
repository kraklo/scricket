use super::component::ComponentEvent;
use super::{GameEvent, Page};

#[derive(Debug, Clone)]
pub enum Event {
    ComponentEvent(ComponentEvent),
    GameEvent(GameEvent),
    LoadGame,
    SaveGame,
    ChangePage(Page),
}
