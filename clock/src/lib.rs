const MINUTES_PER_DAY: i32 = 1440;
const MINUTES_PER_HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock {
    minute_of_day: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self {
            minute_of_day: Self::wrap_minute_of_day(hours * MINUTES_PER_HOUR + minutes),
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        Self {
            minute_of_day: Self::wrap_minute_of_day(self.minute_of_day + minutes),
        }
    }

    fn hour(&self) -> i32 {
        self.minute_of_day / MINUTES_PER_HOUR
    }

    fn minute(&self) -> i32 {
        self.minute_of_day % MINUTES_PER_HOUR
    }

    fn wrap_minute_of_day(minute_of_day: i32) -> i32 {
        // Pending RFC 2169
        (minute_of_day % MINUTES_PER_DAY + MINUTES_PER_DAY) % MINUTES_PER_DAY
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "{:02}:{:02}", self.hour(), self.minute())
    }
}
