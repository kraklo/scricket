use crate::components::ComponentEvent;
use crate::state::{GameEvent, Page};

#[derive(Debug, Clone)]
pub enum Event {
    ComponentEvent(ComponentEvent),
    GameEvent(GameEvent),
    LoadGame,
    SaveGame,
    ChangePage(Page),
    Undo,
}
