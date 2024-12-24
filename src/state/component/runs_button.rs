use super::{AsEvent, ComponentEvent};
use crate::state::event::Event;
use iced::widget::{button, row, text};
use iced::Element;
use macros::AsComponentEvent;

pub struct RunsButton {
    pub runs: u32,
    minimum_runs: u32,
}

impl RunsButton {
    pub fn update(&mut self, event: RunsButtonEvent) {
        match event {
            RunsButtonEvent::Add => self.runs += 1,
            RunsButtonEvent::Subtract => {
                self.runs = if self.runs > self.minimum_runs {
                    self.runs - 1
                } else {
                    self.minimum_runs
                }
            }
        }
    }

    pub fn view<'a>(&'a self) -> Element<'a, Event> {
        return row![
            button("-").on_press(RunsButtonEvent::Subtract.as_event()),
            text(self.runs),
            button("+").on_press(RunsButtonEvent::Add.as_event())
        ]
        .into();
    }
}

impl RunsButton {
    pub fn new(minimum_runs: u32) -> Self {
        return Self {
            runs: minimum_runs,
            minimum_runs,
        };
    }
}

#[derive(Debug, Clone, AsComponentEvent)]
pub enum RunsButtonEvent {
    Add,
    Subtract,
}
