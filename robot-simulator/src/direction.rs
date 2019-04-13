use Direction::*;

#[derive(Debug, PartialEq)]
pub enum Direction { East, North, South, West }

impl Direction {
    pub fn to_left(&self) -> Self {
        match self {
            East => North,
            North => West,
            South => East,
            West => South,
        }
    }

    pub fn to_right(&self) -> Self {
        match self {
            East => South,
            North => East,
            South => West,
            West => North,
        }
    }
}
