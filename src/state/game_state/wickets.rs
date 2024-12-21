use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumIter, Display)]
pub enum HowOut {
    #[strum(to_string = "Did not bat")]
    DidNotBat,
    #[strum(to_string = "Not out")]
    NotOut,
    Bowled,
    #[strum(to_string = "LBW")]
    Lbw,
    Caught,
    #[strum(to_string = "Run out")]
    RunOut,
    Stumped,
    #[strum(to_string = "Hit wicket")]
    HitWicket,
    #[strum(to_string = "Hit ball twice")]
    HitBallTwice,
    #[strum(to_string = "Handled the ball")]
    HandledBall,
    #[strum(to_string = "Obstructed the field")]
    ObstructedField,
    #[strum(to_string = "Timed out")]
    TimedOut,
    #[strum(to_string = "Retired hurt")]
    RetiredHurt,
    #[strum(to_string = "Retired not out")]
    RetiredNotOut,
}
