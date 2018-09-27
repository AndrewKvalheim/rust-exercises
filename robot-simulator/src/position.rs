use crate::direction::Direction;
use crate::direction::Direction::*;

#[derive(Clone)]
pub struct Position(i32, i32);

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position(x, y)
    }

    pub fn toward(&self, direction: &Direction, distance: i32) -> Self {
        match direction {
            East => Position(self.0 + distance, self.1),
            North => Position(self.0, self.1 + distance),
            South => Position(self.0, self.1 - distance),
            West => Position(self.0 - distance, self.1),
        }
    }
}

impl From<Position> for (i32, i32) {
    fn from(position: Position) -> (i32, i32) {
        (position.0, position.1)
    }
}
