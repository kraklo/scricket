use super::component::ComponentEvent;
use super::GameEvent;

#[derive(Debug, Clone)]
pub enum Event {
    ComponentEvent(ComponentEvent),
    GameEvent(GameEvent),
    LoadGame,
    SaveGame,
}
