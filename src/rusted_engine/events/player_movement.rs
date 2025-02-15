use std::sync::{Arc, RwLock};

use glfw::Key;
use nalgebra::Vector3;
use rusted_open::framework::{events::movement, graphics::internal_object::graphics_object::Generic2DGraphicsObject};

use crate::rusted_engine::input::key_states::KeyStates;

// Apply movement based on active keys
pub fn process_player_movement(controlled_obj: Arc<RwLock<Generic2DGraphicsObject>>, key_states: Arc<RwLock<KeyStates>>, delta_time: f32) {
    let move_speed = 0.2;
    let rotation_speed = 2.0;
    let key_states_read = key_states.read().unwrap();

    if key_states_read.is_key_pressed_raw(Key::W) {
        movement::move_object(controlled_obj.clone(), Vector3::new(0.0, 1.0, 0.0), move_speed, delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::S) {
        movement::move_object(controlled_obj.clone(), Vector3::new(0.0, -1.0, 0.0), move_speed, delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::A) {
        movement::move_object(controlled_obj.clone(), Vector3::new(-1.0, 0.0, 0.0), move_speed, delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::D) {
        movement::move_object(controlled_obj.clone(), Vector3::new(1.0, 0.0, 0.0), move_speed, delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::Q) {
        movement::rotate_object(controlled_obj.clone(), rotation_speed*delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::E) {
        movement::rotate_object(controlled_obj.clone(), -rotation_speed*delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::G) {
        controlled_obj.write().unwrap().set_rotation(0.0);
    }
}