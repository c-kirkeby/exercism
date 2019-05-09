use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    const DAY_IN_MINS: i32 = 1440;
    const HOUR_IN_MINS: i32 = 60;
    pub fn new(hours: i32, minutes: i32) -> Self {
        // Convert everything to minutes to simplify our calculations
        let mut total_mins = (hours * Clock::HOUR_IN_MINS + minutes) % Clock::DAY_IN_MINS;

        // If the total number of mins ends up negative, we need to compensate
        // to make it positive.
        if total_mins < 0 {
            total_mins += Clock::DAY_IN_MINS;
        }

        // We get the remainder by diving the number of minutes in a day and
        // divide that remainder by 60 to get the number of hours. Then we
        // get the remainder of division by 60 to get the number of minutes.
        Clock {
            hours: total_mins % Clock::DAY_IN_MINS / Clock::HOUR_IN_MINS,
            minutes: total_mins % Clock::HOUR_IN_MINS,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // Since the new method already deals with overflow, we can leverage
        // it directly to get the new Clock instance.
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
