#[derive(Copy, Clone)]
pub struct Location {
    current: u32,
}

impl Location {
    pub fn new() -> Self {
        Location { current: 0 }
    }

    pub fn current(&self) -> u32 {
        self.current
    }

    pub fn increase(&self) -> Self {
        Location {
            current: self.current + 1,
        }
    }
}
