#![feature(euclidean_division)]

use std::fmt::{self, Display};

const DAY: i32 = 1440;
const HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self::from_minutes(hours * HOUR + minutes)
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Self::from_minutes(self.0 + minutes)
    }

    fn from_minutes(minutes: i32) -> Self {
        Self(minutes.rem_euclid(DAY))
    }

    fn hour(&self) -> i32 {
        self.0 / HOUR
    }

    fn minute(&self) -> i32 {
        self.0 % HOUR
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour(), self.minute())
    }
}
