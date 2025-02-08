use rusted_open::engine::{events::collision::{self, CollisionEvent}, graphics::util::master_graphics_list::MasterGraphicsList};
use crate::game_test::entities::{generic_entity::GenericEntity, util::master_entity_list::MasterEntityList};

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
                            if entity_1.get_weight() > entity_2.get_weight() {
                                self.move_entity_based_on_position(master_graphics_list, &entity_1, &entity_2, 0.05);
                            }
                            // push the entity 1 away from entity 2 based on position
                            if entity_2.get_weight() > entity_1.get_weight() {
                                self.move_entity_based_on_position(master_graphics_list, &entity_2, &entity_1, 0.005);
                            }
                            // push the entities away from eachother based position
                            if entity_1.get_weight() == entity_2.get_weight() {
                                self.move_entity_based_on_position(master_graphics_list, &entity_1, &entity_2, 0.025);
                                self.move_entity_based_on_position(master_graphics_list, &entity_2, &entity_1, 0.025);
                            }
                        }
                    }
                }
            }
        }
    }

    fn move_entity_based_on_position(&self, master_graphics_list: &MasterGraphicsList, unmoved_entity: &GenericEntity, moved_entity: &GenericEntity, push_force: f32) {
        let (entity_1_pos, mut entity_2_pos);
        
        // Get the positions of both entities
        if let Some(entity_1_graphics_object) = master_graphics_list.get_object(unmoved_entity.get_name()) {
            entity_1_pos = entity_1_graphics_object.read().unwrap().get_position();
        } else {
            return; // Handle the case where entity_1 doesn't have a graphics object
        }
    
        if let Some(entity_2_graphics_object) = master_graphics_list.get_object(moved_entity.get_name()) {
            entity_2_pos = entity_2_graphics_object.read().unwrap().get_position();
        } else {
            return; // Handle the case where entity_2 doesn't have a graphics object
        }
    
        // Calculate the positional differences
        let diff_x = entity_1_pos.x - entity_2_pos.x;
        let diff_y = entity_1_pos.y - entity_2_pos.y;
    
        // Compare differences to determine largest direction of movement
        if diff_x.abs() > diff_y.abs() {
            // If X difference is larger, move along X
            if diff_x < 0.0 {
                // Entity 2 is further west, so push east (positive X)
                entity_2_pos.x += push_force; // Apply eastward push
            } else {
                // Entity 2 is further east, so push west (negative X)
                entity_2_pos.x -= push_force; // Apply westward push
            }
        } else {
            // If Y difference is larger, move along Y
            if diff_y < 0.0 {
                // Entity 2 is further north, so push south (positive Y)
                entity_2_pos.y += push_force; // Apply southward push
            } else {
                // Entity 2 is further south, so push north (negative Y)
                entity_2_pos.y -= push_force; // Apply northward push
            }
        }
    
        // Update the position of entity_2 in the graphics object (if needed)
        if let Some(entity_2_graphics_object) = master_graphics_list.get_object(moved_entity.get_name()) {
            let mut entity_2_graphics = entity_2_graphics_object.write().unwrap();
            entity_2_graphics.set_position(entity_2_pos); // Assuming you have a method to update the position
        }
    }
    

    pub fn destroy_object(&self, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, object_name: String) {
        master_entity_list.remove_entity(&object_name);
        master_graphics_list.remove_object(&object_name);
    }
}
