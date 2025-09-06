use std::fmt::Display;

#[derive(PartialEq, Debug)]
pub struct Clock(i32, i32);

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // calculate time in minutes
        let t = hours * 60 + minutes;
        let h = t / 60 % 24;
        let m = t % 60;

        if t < 0 {
            let mut h = 23 + h;
            let mut m = 60 + m;
            
            if h == 24 {
                h = 0;
            }

            if m == 60 {
                h +=1;
                m = 0;
            }
            
            return Clock(h, m);
        }

        Clock(h, m)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // calculate new time in minutes
        let t = self.0 * 60 + self.1 + minutes;
        let h = t / 60 % 24;
        let m = t % 60;
        
        if t < 0 {
            let h = 23 + h;
            let m = 60 + m;
            
            return Clock(h, m);
        }
        
        Clock(h, m)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let hour_padding = if self.0 < 10 { "0" } else {""};
        let min_padding = if self.1 < 10 { "0" } else {""};
        f.write_fmt(format_args!("{}{}:{}{}", hour_padding, self.0, min_padding, self.1))
    }
}
