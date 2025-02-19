use godot::{classes::Label3D, prelude::*};

const DAY: usize = 86400;
const HOUR: usize = 3600;
const MINUTE: usize = 60;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Clock {
    base: Base<Node3D>,
    tick: usize,
    time: usize,
}

#[godot_api]
impl INode3D for Clock {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            tick: 100,
            time: 0,
        }
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
        let mut label = self.base().get_node_as::<Label3D>("Time");
        label.set_text(&self.military_time());
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
