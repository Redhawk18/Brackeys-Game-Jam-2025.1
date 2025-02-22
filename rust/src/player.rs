use std::f32::consts::PI;

use godot::classes::input::MouseMode;
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion};
use godot::global::clamp;
use godot::obj::WithBaseField;
use godot::prelude::*;

const MOUSE_SENSITIVITY: f32 = 0.25;

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    base: Base<CharacterBody3D>,
    camera_input_direction: Vector2,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            camera_input_direction: Vector2::ZERO,
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        let mut input = Input::singleton();
        if event.is_action_pressed("left_click") {
            input.set_mouse_mode(MouseMode::CAPTURED);
        }

        if event.is_action_pressed("ui_cancel") {
            input.set_mouse_mode(MouseMode::VISIBLE);
        }
    }

    fn unhandled_input(&mut self, event: Gd<InputEvent>) {
        let input = Input::singleton();
        if input.get_mouse_mode() == MouseMode::CAPTURED {
            if let Ok(a) = event.try_cast::<InputEventMouseMotion>() {
                self.camera_input_direction = a.get_screen_relative() * MOUSE_SENSITIVITY;
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        let mut pivot = self.base().get_node_as::<Node3D>("Player/Pivot");

        let x = pivot.get_rotation().x + self.camera_input_direction.y * delta as f32;
        let clamp = clamp(
            &x.to_variant(),
            &(-PI / 6.0).to_variant(),
            &(PI / 3.0).to_variant(),
        );
        pivot.rotate_x(clamp.to());

        let y = pivot.get_rotation().y - self.camera_input_direction.x * delta as f32;
        pivot.rotate_y(y);

        self.camera_input_direction = Vector2::ZERO;
    }
}
