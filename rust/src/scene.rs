use godot::{classes::notify::NodeNotification, prelude::*};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct Scene {
    base: Base<Node>,
}

#[godot_api]
impl Scene {}

#[godot_api]
impl INode for Scene {
    fn on_notification(&mut self, _what: NodeNotification) {}

    fn init(base: Base<Node>) -> Self {
        Scene { base }
    }

    fn ready(&mut self) {
        // self.music = Some(self.base().get_node_as("Music"));
    }
}
