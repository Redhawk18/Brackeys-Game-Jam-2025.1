use godot::{classes::ITimer, prelude::*};

const DAY: usize = 86400;
const HOUR: usize = 3600;
const MINUTE: usize = 60;

#[derive(GodotClass)]
#[class(base=Timer)]
pub struct Clock {
    tick: usize,
    time: usize,
}

#[godot_api]
impl ITimer for Clock {
    fn init(_base: Base<Self::Base>) -> Self {
        Clock { tick: 100, time: 0 }
    }

    fn to_string(&self) -> GString {
        self.military_time().into()
    }

    fn process(&mut self, _delta: f64) {
        self.time += self.tick;

        if self.time >= DAY {
            self.time = 0;
        }

        godot_print!("{}", self.time);

        // get label and set text to self.military_time()
        let label = todo!();
    }
}

impl Clock {
    fn military_time(&self) -> String {
        let hour = self.time / HOUR;
        let minute = self.time / MINUTE;

        String::from(format!("{:02}:{:02}", hour, minute))
    }

    fn standard_time(&self) -> String {
        let hour = self.time / HOUR;
        let minute = self.time / MINUTE;

        let mut string = format!("{:02}:{:02}", hour, minute);
        if hour <= 12 {
            string.push_str(" am");
        } else {
            string.push_str(" pm");
        }

        string
    }
}
