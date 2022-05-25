use crate::{Name, Position};

#[derive(Debug, Clone)]
pub struct RaceResult {
    name: Name,
    position: Position,
}

impl RaceResult {
    pub fn new(name: &Name, position: Position) -> Self {
        RaceResult {
            name: name.clone(),
            position,
        }
    }

    pub fn name(&self) -> Name {
        self.name.clone()
    }

    pub fn position(&self) -> Position {
        self.position.clone()
    }
}
