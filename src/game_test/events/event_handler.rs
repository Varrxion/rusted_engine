use rusted_open::engine::{events::collision::{self, CollisionEvent}, graphics::util::master_graphics_list::MasterGraphicsList};
use crate::game_test::entities::{self, util::master_entity_list::MasterEntityList};

pub struct EventHandler;

impl EventHandler {
    /// Call the collision check on all objects whose entity indicates collision should be actively checked.
    pub fn check_entity_collisions(&self, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList) -> Vec<collision::CollisionEvent> {
        let entities = master_entity_list.get_entities();
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
            let events = collision::check_collisions(master_graphics_list, &name);
            collision_events.extend(events); // Collect all the collision events
        }

        collision_events
    }

    pub fn handle_collision_events(&self, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, collision_events: Vec<CollisionEvent>) {
        for collision_event in collision_events {
            if let Some(entity_1) = master_entity_list.get_entity(&collision_event.object_name_1) {
                if let Ok(entity_1) = entity_1.read() {
                    if let Some(entity_2) = master_entity_list.get_entity(&collision_event.object_name_2) {
                        if let Ok(entity_2) = entity_2.read() {
                            if entity_1.can_destroy() && entity_2.is_destructible() && entity_1.get_weight() >= entity_2.get_weight() {
                                self.destroy_object(master_entity_list, master_graphics_list, entity_2.get_name().to_owned())
                            }
                            if entity_2.can_destroy() && entity_1.is_destructible() && entity_2.get_weight() >= entity_1.get_weight() {
                                self.destroy_object(master_entity_list, master_graphics_list, entity_1.get_name().to_owned());
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn destroy_object(&self, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, object_name: String) {
        master_entity_list.remove_entity(&object_name);
        master_graphics_list.remove_object(&object_name);
    }
}
