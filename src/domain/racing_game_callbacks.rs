use crate::domain::position::Position;

pub trait RacingGameCallback {
    fn on_raced(&self, positions: Vec<Position>);
}
