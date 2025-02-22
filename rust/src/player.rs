use std::f32::consts::PI;

use godot::classes::input::MouseMode;
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion};
use godot::obj::WithBaseField;
use godot::prelude::*;
use num::clamp;

const MOUSE_SENSITIVITY: f32 = 0.25;
// const MOUSE_SENSITIVITY: f32 = 0.000_000_1;
const SPEED: f32 = 8.0;
const ACCERATION: f32 = 20.0;

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
        if Input::singleton().get_mouse_mode() == MouseMode::CAPTURED {
            if let Ok(movement) = event.try_cast::<InputEventMouseMotion>() {
                self.camera_input_direction.x =
                    -movement.get_screen_relative().x * MOUSE_SENSITIVITY;
                self.camera_input_direction.y =
                    -movement.get_screen_relative().y * MOUSE_SENSITIVITY;
            }
        }
    }

    fn physics_process(&mut self, delta: f64) {
        // let mut pivot = self.base().get_node_as::<Node3D>("Player/Pivot");
        let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
        let camera = self
            .base()
            .get_node_as::<Node3D>("Pivot/SpringArm3D/Camera3D");

        // _camera_pivot.rotation.x += _camera_input_direction.y * delta
        let x = pivot.get_rotation().x + self.camera_input_direction.y * delta as f32;
        // _camera_pivot.rotation.x = clamp(_camera_pivot.rotation.x, tilt_lower_limit, tilt_upper_limit)
        let clamp = clamp(x, -PI / 6.0, PI / 3.0);
        pivot.rotate_x(clamp);

        // _camera_pivot.rotation.y += _camera_input_direction.x * delta
        let y = pivot.get_rotation().y - self.camera_input_direction.x * delta as f32;
        pivot.rotate_y(y);

        // _camera_input_direction = Vector2.ZERO
        self.camera_input_direction = Vector2::ZERO;

        // var raw_input := Input.get_vector("move_left", "move_right", "move_up", "move_down", 0.4)
        let raw = Input::singleton().get_vector(
            "move_left",
            "move_right",
            "move_forward",
            "move_backward",
        );
        // var forward := _camera.global_basis.z
        let forward = camera.get_global_basis().col_c();
        // var right := _camera.global_basis.x
        let right = camera.get_global_basis().col_a();

        let velocity = self.base().get_velocity();
        // var move_direction := forward * raw_input.y + right * raw_input.x
        let mut direction = forward * raw.y + right * raw.x;
        // move_direction.y = 0.0
        direction.y = 0.0;
        // move_direction = move_direction.normalized()
        dbg!(direction);
        direction = direction.normalized(); // <---
        direction = direction.move_toward(direction, SPEED);

        // velocity = velocity.move_toward(move_direction * move_speed, acceleration * delta)
        self.base_mut()
            .set_velocity(velocity.move_toward(direction, ACCERATION * delta as f32));
        // velocity = velocity.move_toward(move_direction * move_speed, acceleration * delta)
        self.base_mut().move_and_slide();
    }
}
