#[derive(PartialEq, Debug)]
pub struct Position {
    position: u32,
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
