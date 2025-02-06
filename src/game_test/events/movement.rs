use std::sync::{Arc, RwLock};
use nalgebra::Vector3;
use rusted_open::engine::events::movement;
use rusted_open::engine::graphics::internal_object::graphics_object::Generic2DGraphicsObject;

pub struct Movement;

impl Movement {
    /// Moves the object in a specified direction with a given speed and delta time.
    pub fn move_object(object: Arc<RwLock<Generic2DGraphicsObject>>, direction: Vector3<f32>, speed: f32, delta_time: f32) {
        movement::move_object(object, direction, speed, delta_time);
    }

    /// Rotates the object by a specified angle (in radians).
    pub fn rotate_object(object: Arc<RwLock<Generic2DGraphicsObject>>, angle: f32) {
        movement::rotate_object(object, angle);
    }
}
