use crate::domain::position::Position;
use crate::domain::race_result::RaceResult;

pub trait RacingGameCallback {
    fn on_raced(&self, positions: Vec<Position>);
    fn on_raced2(&self, result: Vec<RaceResult>);
}
