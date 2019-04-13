use std::fmt::{self, Display};

const DAY: i32 = 1440;
const HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self(Self::wrap_around_day(HOUR * hours + minutes))
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Self(Self::wrap_around_day(self.0 + minutes))
    }

    fn hour(&self) -> i32 {
        self.0 / HOUR
    }

    fn minute(&self) -> i32 {
        self.0 % HOUR
    }

    fn wrap_around_day(minutes: i32) -> i32 {
        // Pending RFC 2169
        (minutes % DAY + DAY) % DAY
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour(), self.minute())
    }
}
