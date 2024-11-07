mod state;

use state::State;

fn main() -> iced::Result {
    iced::run("scricket", State::update, State::view)
}
