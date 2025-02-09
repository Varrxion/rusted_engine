use std::sync::{Arc, RwLock};

use nalgebra::Vector3;
use rusted_open::framework::graphics::util::master_graphics_list::MasterGraphicsList;

use crate::rusted_engine::entities::util::master_entity_list::MasterEntityList;

use super::collision::{self, CollisionEvent};

pub struct EventHandler {
    master_entity_list: Arc<RwLock<MasterEntityList>>,
    master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
}

impl EventHandler {
    // We will let the event handler initialize with both master lists
    pub fn new(
        master_entity_list: Arc<RwLock<MasterEntityList>>,
        master_graphics_list: Arc<RwLock<MasterGraphicsList>>,
    ) -> Self {
        Self {
            master_entity_list,
            master_graphics_list,
        }
    }

    /// Call the collision check on all objects whose entity indicates collision should be actively checked.
    pub fn check_entity_collisions(&self) -> Vec<collision::CollisionEvent> {
        let entities = self.master_entity_list.read().unwrap().get_entities();
        let entities = entities.read().unwrap();
        
        // Iterate over each entity in the master entity list
        let mut relevant_names = Vec::new();
        for entity in entities.values() {
            if let Ok(entity) = entity.read() {
                // Check if the entity is active
                if entity.has_active_collision() {
                    // If active, add the entity's name to the list for collision checking
                    relevant_names.push(entity.get_name().to_owned());
                }
            }
        }

        // Now check collisions for each relevant entity
        let mut collision_events = Vec::new();
        for name in relevant_names {
            // Pass each active entity's name to the collision check function
            let events = collision::check_collisions(
                &self.master_entity_list.read().unwrap(),
                &self.master_graphics_list.read().unwrap(),
                &name,
            );
            collision_events.extend(events); // Collect all the collision events
        }

        collision_events
    }

    pub fn handle_collision_events(&self, collision_events: Vec<CollisionEvent>) {
        for collision_event in collision_events {
            if let Some(entity_1) = self.master_entity_list.read().unwrap().get_entity(&collision_event.object_name_1) {
                if let Ok(entity_1) = entity_1.read() {
                    if let Some(entity_2) = self.master_entity_list.read().unwrap().get_entity(&collision_event.object_name_2) {
                        if let Ok(entity_2) = entity_2.read() {
                            if entity_1.can_destroy() && entity_2.is_destructible() && entity_1.get_weight() >= entity_2.get_weight() {
                                self.destroy_object(entity_2.get_name().to_owned())
                            }
                            if entity_2.can_destroy() && entity_1.is_destructible() && entity_2.get_weight() >= entity_1.get_weight() {
                                self.destroy_object(entity_1.get_name().to_owned());
                            }
                            if entity_1.get_weight() > entity_2.get_weight() {
                                collision::collision_move_entity_based_on_position(
                                    &self.master_graphics_list.read().unwrap(),
                                    &entity_1,
                                    &entity_2,
                                    0.05,
                                );
                            }
                            // push the entity 1 away from entity 2 based on position
                            if entity_2.get_weight() > entity_1.get_weight() {
                                collision::collision_move_entity_based_on_position(
                                    &self.master_graphics_list.read().unwrap(),
                                    &entity_2,
                                    &entity_1,
                                    0.005,
                                );
                            }
                            // push the entities away from each other based on position
                            if entity_1.get_weight() == entity_2.get_weight() {
                                collision::collision_move_entity_based_on_position(
                                    &self.master_graphics_list.read().unwrap(),
                                    &entity_1,
                                    &entity_2,
                                    0.025,
                                );
                                collision::collision_move_entity_based_on_position(
                                    &self.master_graphics_list.read().unwrap(),
                                    &entity_2,
                                    &entity_1,
                                    0.025,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn destroy_object(&self, object_name: String) {
        self.master_entity_list.write().unwrap().remove_entity(&object_name);
        self.master_graphics_list.write().unwrap().remove_object(&object_name);
    }

    pub fn homebringer_sequence(&self) {
        if let Some(player_object) = self.master_graphics_list.read().unwrap().get_object("testscene_playersquare") {
            player_object.write().unwrap().set_position(Vector3::new(0.0, 0.0, 0.0));
        }
    }
}
