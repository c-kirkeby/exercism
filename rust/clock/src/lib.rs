use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    minutes: i32,
    hours: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let min_quotient = minutes / 60;
        let min_remainder = minutes % 60;
        Clock {
            hours: (hours + min_quotient) % 24,
            minutes: min_remainder,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let min_quotient = self.minutes + minutes / 60;
        let min_remainder = self.minutes + minutes % 60;
        Clock {
            hours: (self.hours + min_quotient) % 24,
            minutes: min_remainder,
        }
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (self.hours > 9, self.minutes > 9) {
            (true, true) => write!(f, "{}:{}", self.hours, self.minutes),
            (true, false) => write!(f, "{}:0{}", self.hours, self.minutes),
            (false, true) => write!(f, "0{}:{}", self.hours, self.minutes),
            (false, false) => write!(f, "0{}:0{}", self.hours, self.minutes),
        }
    }
}
