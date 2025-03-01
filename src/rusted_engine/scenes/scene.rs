use std::sync::{Arc, RwLock};

use nalgebra::Vector2;
use rusted_open::framework::graphics::internal_object::graphics_object::Generic2DGraphicsObject;

use crate::rusted_engine::entities::generic_entity::GenericEntity;

pub struct Scene {
    entities: Vec<Arc<RwLock<GenericEntity>>>,
    graphics_objects: Vec<Arc<RwLock<Generic2DGraphicsObject>>>,
    gravity: Vector2<f32>,
    terminal_velocity: Vector2<f32>,
}

impl Scene {
    pub fn new(gravity: Vector2<f32>, terminal_velocity: Vector2<f32>) -> Self {
        Scene {
            entities: Vec::new(),
            graphics_objects: Vec::new(),
            gravity,
            terminal_velocity,
        }
    }

    pub fn add_graphics_object(&mut self, obj: Arc<RwLock<Generic2DGraphicsObject>>) {
        self.graphics_objects.push(obj);
    }

    pub fn get_graphics_objects(&self) -> &Vec<Arc<RwLock<Generic2DGraphicsObject>>> {
        &self.graphics_objects
    }

    pub fn add_entity(&mut self, obj: Arc<RwLock<GenericEntity>>) {
        self.entities.push(obj);
    }

    pub fn get_entities(&self) -> &Vec<Arc<RwLock<GenericEntity>>> {
        &self.entities
    }

    pub fn set_gravity(&mut self, gravity: Vector2<f32>) {
        self.gravity = gravity;
    }

    pub fn get_gravity(&self) -> Vector2<f32> {
        self.gravity
    }

    pub fn set_terminal_velocity(&mut self, terminal_velocity: Vector2<f32>) {
        self.terminal_velocity = terminal_velocity;
    }

    pub fn get_terminal_velocity(&self) -> Vector2<f32> {
        self.terminal_velocity
    }
}
