use godot::prelude::*;

mod clock;
mod player;
mod scene;

struct Machine;

#[gdextension]
unsafe impl ExtensionLibrary for Machine {}
