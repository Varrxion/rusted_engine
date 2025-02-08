use std::sync::{Arc, RwLock};

use rusted_open::engine::graphics::internal_object::graphics_object::Generic2DGraphicsObject;

pub struct Scene {
    objects: Vec<Arc<RwLock<Generic2DGraphicsObject>>>, // Use Generic2DGraphicsObject for static objects
}

impl Scene {
    pub fn new() -> Self {
        Scene {
            objects: Vec::new(),
        }
    }

    // Add static Generic2DGraphicsObject to the scene
    pub fn add_object(&mut self, obj: Arc<RwLock<Generic2DGraphicsObject>>) {
        self.objects.push(obj);
    }

    pub fn get_objects(&self) -> &Vec<Arc<RwLock<Generic2DGraphicsObject>>> {
        &self.objects
    }
}
