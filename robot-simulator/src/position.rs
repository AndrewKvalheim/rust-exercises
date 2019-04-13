use crate::direction::Direction::{self, *};

#[derive(Clone)]
pub struct Position(i32, i32);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn toward(&self, direction: &Direction, distance: i32) -> Self {
        match direction {
            East => Self(self.0 + distance, self.1),
            North => Self(self.0, self.1 + distance),
            South => Self(self.0, self.1 - distance),
            West => Self(self.0 - distance, self.1),
        }
    }
}

impl From<Position> for (i32, i32) {
    fn from(position: Position) -> (i32, i32) {
        (position.0, position.1)
    }
}
