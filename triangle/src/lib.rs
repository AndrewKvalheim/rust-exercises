extern crate num;

use num::Num;

pub struct Triangle<T>(T, T, T);

impl<T: Copy + Num + PartialOrd> Triangle<T> {
    pub fn build(sides: [T; 3]) -> Option<Self> {
        if Self::validate(&sides) {
            Some(Triangle(sides[0], sides[1], sides[2]))
        } else {
            None
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.0 == self.1 && self.0 == self.2
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.2 == self.0
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.1 != self.2 && self.2 != self.0
    }

    fn validate(sides: &[T; 3]) -> bool {
        !sides[0].is_zero()
            && !sides[1].is_zero()
            && !sides[2].is_zero()
            && sides[0] + sides[1] > sides[2]
            && sides[1] + sides[2] > sides[0]
            && sides[2] + sides[0] > sides[1]
    }
}
