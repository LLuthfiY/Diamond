#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock{
    hour:i32,
    minute:i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let mut h = (hours+(minutes/60))%24;
        let mut m = minutes%60;
        if m < 0 {
            m = 60 + m;
            h = h - 1;
        }
        if h<0{
            h = 24 + h;
        }

        Clock{hour : h , minute : m}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let mut h = (self.hour + (minutes + self.minute)/60)%24;
        let mut m = (self.minute + (minutes))%60;
        
        if m<0{
            m = 60 + m;
            h = h - 1;
        }
        if h<0{
            h = 24 + h;
        }
        Clock{hour : h,
        minute : m}
    }

    pub fn to_string(&self) -> String {
        format!("{:02}:{:02}", &self.hour , &self.minute)
    }
}

