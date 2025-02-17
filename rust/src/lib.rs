use godot::prelude::*;

mod clock;
mod scene;

struct Machine;

#[gdextension]
unsafe impl ExtensionLibrary for Machine {}
