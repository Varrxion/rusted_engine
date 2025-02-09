use std::sync::{Arc, RwLock};

use rusted_open::framework::graphics::internal_object::graphics_object::Generic2DGraphicsObject;

use crate::rusted_engine::entities::generic_entity::GenericEntity;

pub struct Scene {
    entities: Vec<Arc<RwLock<GenericEntity>>>,
    graphics_objects: Vec<Arc<RwLock<Generic2DGraphicsObject>>>,
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            entities: Vec::new(),
            graphics_objects: Vec::new(),
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
}
