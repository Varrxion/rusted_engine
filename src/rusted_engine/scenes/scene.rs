use std::sync::{Arc, RwLock};

use nalgebra::Vector2;
use rusted_open::framework::graphics::internal_object::graphics_object::Generic2DGraphicsObject;

use crate::rusted_engine::{entities::generic_entity::GenericEntity, events::triggers::SceneTrigger};

use super::scene_properties::SceneProperties;

pub struct Scene {
    entities: Vec<Arc<RwLock<GenericEntity>>>,
    graphics_objects: Vec<Arc<RwLock<Generic2DGraphicsObject>>>,
    properties: SceneProperties,
    scene_triggers: Vec<SceneTrigger>, // Empty vector if no triggers in the scene
}

impl Scene {
    pub fn new(properties: SceneProperties, scene_triggers: Vec<SceneTrigger>) -> Self {
        Scene {
            entities: Vec::new(),
            graphics_objects: Vec::new(),
            properties,
            scene_triggers,
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

    // properties

    pub fn get_properties(&self) -> SceneProperties {
        self.properties.clone()
    }

    pub fn set_properties(&mut self, properties: SceneProperties) {
        self.properties = properties;
    }

    pub fn set_gravity(&mut self, gravity: Vector2<f32>) {
        self.properties.set_gravity(gravity);
    }

    pub fn get_gravity(&self) -> Vector2<f32> {
        self.properties.get_gravity()
    }

    pub fn set_terminal_velocity(&mut self, terminal_velocity: Vector2<f32>) {
        self.properties.set_terminal_velocity(terminal_velocity);
    }

    pub fn get_terminal_velocity(&self) -> Vector2<f32> {
        self.properties.get_terminal_velocity()
    }

    // triggers

    pub fn get_triggers(&self) -> Vec<SceneTrigger> {
        self.scene_triggers.clone()
    }

    pub fn set_triggers(&mut self, scene_triggers: Vec<SceneTrigger>) {
        self.scene_triggers = scene_triggers;
    }
}
