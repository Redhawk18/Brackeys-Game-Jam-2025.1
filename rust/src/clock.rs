use godot::{classes::Label3D, prelude::*};
use rand::{prelude::*, rng};

const DAY: usize = 24;
const HOUR: usize = 60;
// const MINUTE: usize = 60;
// const TIMEOUT: usize = 1000;
const TIMEOUT: usize = 100;
const START: usize = 22;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Clock {
    base: Base<Node3D>,
    /// Time increases by 0 till this value
    tick: usize,
    hour: usize,
    minute: usize,
    timeout: usize,
    rng: ThreadRng,
}

#[godot_api]
impl INode3D for Clock {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            tick: 3,
            hour: START,
            minute: 0,
            timeout: TIMEOUT,
            rng: rng(),
        }
    }

    fn to_string(&self) -> GString {
        self.standard_time().into()
    }

    fn process(&mut self, _delta: f64) {
        if TIMEOUT >= self.timeout {
            self.timeout += 1;
            return;
        } else {
            self.timeout = 0;
        }

        self.minute += self.rng.random_range(0..self.tick);

        if self.hour >= DAY {
            self.hour = 0;
            self.minute = 0;
        }

        if self.minute >= HOUR {
            self.hour += 1;
            self.minute = 0;
        }

        let mut label = self.base().get_node_as::<Label3D>("Clock/Time");
        label.set_text(&self.military_time());
    }
}

impl Clock {
    fn military_time(&self) -> String {
        String::from(format!("{:02}:{:02}", self.hour, self.minute))
    }

    fn standard_time(&self) -> String {
        let hour = self.hour % 12;
        let mut string = format!("{:02}:{:02}", hour, self.minute);
        if self.hour <= 12 {
            string.push_str(" am");
        } else {
            string.push_str(" pm");
        }

        string
    }
}
