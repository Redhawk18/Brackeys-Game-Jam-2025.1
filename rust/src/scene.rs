use std::default;

use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Node3D)]
pub struct Scene {
    base: Base<Node3D>,
    music: Option<Gd<AudioStreamPlayer>>,
}

#[godot_api]
impl Scene {}

#[godot_api]
impl INode3D for Scene {
    fn init(base: Base<Node3D>) -> Self {
        Scene { base, music: None }
    }

    fn ready(&mut self) {
        self.music = Some(self.base().get_node_as("Music"));
    }
}
