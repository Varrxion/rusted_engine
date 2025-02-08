use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::game_test::entities::generic_entity::GenericEntity;

pub struct MasterEntityList {
    entities: Arc<RwLock<HashMap<String, Arc<RwLock<GenericEntity>>>>>,
}

impl MasterEntityList {
    /// Initialize a new MasterEntityList
    pub fn new() -> Self {
        MasterEntityList {
            entities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add an entity to the list using its name as the key
    pub fn add_entity(&self, entity: Arc<RwLock<GenericEntity>>) {
        let binding = entity.read().unwrap();
        let name = binding.get_name();
        let mut entities = self.entities.write().unwrap();
        entities.insert(name.to_owned(), entity.clone());
    }
    
    /// Get an entity by name
    pub fn get_entity(&self, name: &str) -> Option<Arc<RwLock<GenericEntity>>> {
        let entities = self.entities.read().unwrap();
        entities.get(name).cloned()
    }

    /// Returns a pointer to the entire entity list
    pub fn get_entities(&self) -> Arc<RwLock<HashMap<String, Arc<RwLock<GenericEntity>>>>> {
        Arc::clone(&self.entities)
    }

    /// Remove an entity by name
    pub fn remove_entity(&self, name: &str) {
        let mut entities = self.entities.write().unwrap();
        entities.remove(name);
    }

    /// Remove all entities from the list
    pub fn remove_all(&self) {
        let mut entities = self.entities.write().unwrap();
        entities.clear();
    }

    /// Debug print all entities
    pub fn debug_all(&self) {
        let entities = self.entities.read().unwrap();
        for entity in entities.values() {
            if let Ok(entity) = entity.read() {
                entity.print_debug();
            }
        }
    }
}
