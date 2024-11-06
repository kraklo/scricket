mod game_state;

use game_state::{Event, GameState};
use iced::widget::{button, text};
use iced::Element;

fn main() -> iced::Result {
    iced::run("scricket", GameState::update, GameState::view)
}
