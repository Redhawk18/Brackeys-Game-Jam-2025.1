use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Scene {
    base: Base<Node3D>,
}

#[godot_api]
impl Scene {}

#[godot_api]
impl INode3D for Scene {
    fn init(base: Base<Node3D>) -> Self {
        Scene { base }
    }

    fn ready(&mut self) {
        // self.music = Some(self.base().get_node_as("Music"));
    }
}
