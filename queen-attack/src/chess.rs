use std::ops::Sub;

pub trait Piece {
    fn can_attack<T: Piece>(&self, other: &T) -> bool;

    fn position(&self) -> &Position;
}

pub struct Position(i32, i32);

impl Position {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        Some(Self(Self::validate(file)?, Self::validate(rank)?))
    }

    fn validate(component: i32) -> Option<i32> {
        Some(component).filter(|&c| 0 <= c && c < 8)
    }
}

impl Sub for &Position {
    type Output = (i32, i32);

    fn sub(self, other: Self) -> Self::Output {
        (self.0 - other.0, self.1 - other.1)
    }
}
