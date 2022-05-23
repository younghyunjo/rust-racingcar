use crate::domain::position::Position;
use crate::domain::race_result::RaceResult;

pub trait RacingGameCallback {
    fn on_raced(&self, result: Vec<RaceResult>);
}
