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

impl Position {
    pub fn new() -> Position {
        Position { position: 0 }
    }

    pub fn with_position(position: u32) -> Position {
        Position { position }
    }

    pub fn increase(&self) -> Position {
        Position::with_position(self.position + 1)
    }
}
