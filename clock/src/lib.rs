// Calculates the least nonnegative remainder of self (mod rhs).
// This is done as if by the Euclidean division algorithm – given r = self.rem_euclid(rhs), 
// self = rhs * self.div_euclid(rhs) + r, and 0 <= r < abs(rhs).
// https://doc.rust-lang.org/std/primitive.i32.html#method.rem_euclid

use std::fmt::{Display, Formatter, Result};
const MINUTES_PER_DAY: i32 = 1440;
const MINUTES_PER_HOUR: i32 = 60;
#[derive(Debug, PartialEq, PartialOrd)]
pub struct Clock { minutes: i32, }
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Self { minutes: ((hours * MINUTES_PER_HOUR) + minutes).rem_euclid(MINUTES_PER_DAY), }
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self { minutes: (self.minutes + minutes).rem_euclid(MINUTES_PER_DAY), }
    }
}
impl Display for Clock {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(
            f,
            "{:02}:{:02}",
            self.minutes / MINUTES_PER_HOUR,
            self.minutes % MINUTES_PER_HOUR
        )
    }
}