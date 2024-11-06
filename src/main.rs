mod game_state;

use game_state::{Event, GameState};
use iced::widget::{button, text};
use iced::Element;

fn main() -> iced::Result {
    iced::run("scricket", update, view)
}

fn update(game_state: &mut GameState, event: Event) {
    match event {
        Event::Runs(runs) => game_state.team_a.runs += runs,
        _ => (),
    }
}

fn view(game_state: &GameState) -> Element<Event> {
    button(text(game_state.team_a.runs))
        .on_press(Event::Runs(1))
        .into()
}
