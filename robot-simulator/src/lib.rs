mod direction;
mod position;

pub use direction::Direction;
use position::Position;

pub struct Robot {
    direction: Direction,
    position: Position,
}

impl Robot {
    pub fn new(x: i32, y: i32, direction: Direction) -> Self {
        Self {
            direction,
            position: Position::new(x, y),
        }
    }

    pub fn advance(self) -> Self {
        Self {
            position: self.position.toward(&self.direction, 1),
            ..self
        }
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }

    pub fn instructions(self, instructions: &str) -> Self {
        instructions
            .chars()
            .fold(self, |robot, instruction| match instruction {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => unimplemented!("instruction {}", instruction),
            })
    }

    pub fn position(&self) -> (i32, i32) {
        self.position.clone().into()
    }

    pub fn turn_left(self) -> Self {
        Self {
            direction: self.direction.to_left(),
            ..self
        }
    }

    pub fn turn_right(self) -> Self {
        Self {
            direction: self.direction.to_right(),
            ..self
        }
    }
}
