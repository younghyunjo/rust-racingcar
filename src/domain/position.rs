use std::convert::From;
use std::fmt::{Debug, Display, Formatter};

#[derive(PartialEq, Debug, Clone)]
pub struct Position {
    position: u32,
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.position)
    }
}

impl From<u32> for Position {
    fn from(poistion: u32) -> Self {
        Position { position: poistion }
    }
}

impl From<Position> for u32 {
    fn from(p: Position) -> Self {
        p.position
    }
}

impl Position {
    pub fn new() -> Position {
        Position { position: 0 }
    }

    pub fn increase(&self) -> Position {
        Position::from(self.position + 1)
    }

    pub fn position(&self) -> u32 {
        self.position
    }
}
