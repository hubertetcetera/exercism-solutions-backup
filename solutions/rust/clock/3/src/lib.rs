use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Clock(i32, i32);

const DAY: i32 = 1440;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // calculate time in minutes
        let t = hours * 60 + minutes;
        // normalize to [0, 1440)
        let t = t.rem_euclid(DAY);
        let h = t / 60;
        let m = t % 60;

        Clock(h, m)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // calculate new time in minutes
        let t = self.0 * 60 + self.1 + minutes;
        // normalize to [0, 1440)
        let t = t.rem_euclid(DAY);
        let h = t / 60;
        let m = t % 60;

        Clock(h, m)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.0, self.1)
    }
}
