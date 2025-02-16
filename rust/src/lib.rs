use godot::prelude::*;

mod scene;
mod time;

struct Machine;

#[gdextension]
unsafe impl ExtensionLibrary for Machine {}
