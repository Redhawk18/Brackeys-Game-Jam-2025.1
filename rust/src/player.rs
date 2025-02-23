use godot::classes::input::MouseMode;
use godot::classes::{CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion};
use godot::obj::WithBaseField;
use godot::prelude::*;
use num::clamp;

const MOUSE_SENSITIVITY: f32 = 0.25;
const SPEED: f32 = 20.0;
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
        let mut pivot = self.base().get_node_as::<Node3D>("Pivot");
        let camera = self
            .base()
            .get_node_as::<Node3D>("Pivot/SpringArm3D/Camera3D");

        pivot.rotate_x(-self.camera_input_direction.y * delta as f32);
        pivot.rotate_y(-clamp(self.camera_input_direction.x, -90.0, 90.0) * delta as f32);
        pivot.rotate_z(0.0);

        self.camera_input_direction = Vector2::ZERO;

        let raw = Input::singleton().get_vector(
            "move_left",
            "move_right",
            "move_forward",
            "move_backward",
        );
        let forward = camera.get_global_basis().col_c();
        let right = camera.get_global_basis().col_a();

        let velocity = self.base().get_velocity();
        let mut direction = forward * raw.y + right * raw.x;
        direction.y = 0.0;
        direction = direction.normalized_or_zero(); // <---
        direction = direction.move_toward(direction, SPEED);

        self.base_mut()
            .set_velocity(velocity.move_toward(direction, ACCERATION * delta as f32));
        self.base_mut().move_and_slide();
    }
}
