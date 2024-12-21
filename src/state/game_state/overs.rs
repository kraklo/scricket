use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Overs {
    pub overs: u32,
    pub balls: u32,
}

impl Overs {
    pub fn new() -> Self {
        Overs { overs: 0, balls: 0 }
    }

    pub fn add_ball(&mut self) {
        self.balls += 1;
    }

    pub fn add_ball_bowler(&mut self) {
        self.balls += 1;

        if self.balls == 6 {
            self.overs += 1;
            self.balls = 0;
        }
    }

    pub fn end_over(&mut self) {
        self.balls = 0;
        self.overs += 1;
    }
}
