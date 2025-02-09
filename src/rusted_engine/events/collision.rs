use std::sync::{Arc, RwLock};

use rusted_open::framework::graphics::{internal_object::graphics_object::Generic2DGraphicsObject, util::master_graphics_list::MasterGraphicsList};

use crate::rusted_engine::{audio::audio_manager::{AudioManager, AudioType}, entities::{generic_entity::{CollisionMode, GenericEntity}, util::master_entity_list::MasterEntityList}};

#[derive(Debug, PartialEq)]
pub struct CollisionEvent {
    pub object_name_1: String,
    pub object_name_2: String,
}

pub fn check_active_entity_collisions(master_entity_list: Arc<RwLock<MasterEntityList>>, master_graphics_list: Arc<RwLock<MasterGraphicsList>>) -> Vec<CollisionEvent> {
    let entities = master_entity_list.read().unwrap().get_entities();
    let entities = entities.read().unwrap();

    let mut relevant_names = Vec::new();
    for entity in entities.values() {
        if let Ok(entity) = entity.read() {
            if entity.has_active_collision() {
                relevant_names.push(entity.get_name().to_owned());
            }
        }
    }

    let mut collision_events = Vec::new();
    for name in relevant_names {
        let events = check_collisions(&master_entity_list.read().unwrap(), &master_graphics_list.read().unwrap(), &name);
        collision_events.extend(events);
    }

    collision_events
}

pub fn handle_collision_events(collision_events: Vec<CollisionEvent>, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, audio_manager: &AudioManager) {
    for collision_event in collision_events {
        if let Some(entity_1) = master_entity_list.get_entity(&collision_event.object_name_1) {
            if let Ok(entity_1) = entity_1.read() {
                if let Some(entity_2) = master_entity_list.get_entity(&collision_event.object_name_2) {
                    if let Ok(entity_2) = entity_2.read() {
                        if entity_1.can_destroy() && entity_2.is_destructible() && entity_1.get_weight() >= entity_2.get_weight() {
                            master_entity_list.remove_entity(&entity_2.get_name());
                            master_graphics_list.remove_object(&entity_2.get_name());
                        }
                        if entity_2.can_destroy() && entity_1.is_destructible() && entity_2.get_weight() >= entity_1.get_weight() {
                            master_entity_list.remove_entity(&entity_1.get_name());
                            master_graphics_list.remove_object(&entity_1.get_name());
                        }
                        if entity_1.get_weight() > entity_2.get_weight() {
                            collision_move_entity_based_on_position(
                                master_graphics_list, &entity_1, &entity_2, 0.05,
                            );
                        }
                        if entity_2.get_weight() > entity_1.get_weight() {
                            collision_move_entity_based_on_position(
                                master_graphics_list, &entity_2, &entity_1, 0.005,
                            );
                        }
                        if entity_1.get_weight() == entity_2.get_weight() {
                            collision_move_entity_based_on_position(
                                master_graphics_list, &entity_1, &entity_2, 0.025,
                            );
                            collision_move_entity_based_on_position(
                                master_graphics_list, &entity_2, &entity_1, 0.025,
                            );
                        }
                        let entity_1_collision_sound = entity_1.get_collision_sound();
                        let entity_2_collision_sound = entity_2.get_collision_sound();
                        if entity_2_collision_sound != "" || entity_2_collision_sound != "null" || entity_2_collision_sound != "none" {
                            audio_manager.enqueue_audio(entity_2_collision_sound, AudioType::Sound, 0.4, false);
                        } // Prioritize the target (entity_2) of the collision for audio sound, rather than the originator of the collision (may flip this around later)
                        else if entity_1_collision_sound != "" || entity_1_collision_sound != "null" || entity_1_collision_sound != "none" {
                            audio_manager.enqueue_audio(entity_1_collision_sound, AudioType::Sound, 0.4, false);
                        }
                    }
                }
            }
        }
    }
}

pub fn check_collisions(master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, object_name: &str) -> Vec<CollisionEvent> {
    let mut collision_events = Vec::new(); // Vector to hold collision events

    if let Some(object_1) = master_graphics_list.get_object(object_name) {
        let object_1_read = object_1.read().unwrap(); // Access the object through RwLock

        // Iterate over all objects in the MasterGraphicsList
        let all_objects = master_graphics_list.get_objects(); // Get the read-only reference
        for (name, object_2) in all_objects.read().unwrap().iter() {
            // Skip the object being checked against itself
            if name == &object_name {
                continue;
            }

            let object_2_read = object_2.read().unwrap(); // Lock for reading

            // Check for collision
            if is_colliding(object_1_read.get_name().to_owned(), object_2_read.get_name().to_owned(), master_entity_list, master_graphics_list) {
                // Create a CollisionEvent and push it into the vector
                collision_events.push(CollisionEvent {
                    object_name_1: object_name.to_string(),
                    object_name_2: name.clone(),
                });
            }
        }
    } else {
        println!("No object found with name: {}", object_name);
    }

    collision_events // Return the vector of collision events
}

pub fn is_colliding_aabb(object_1_read: &Generic2DGraphicsObject,  object_2_read: &Generic2DGraphicsObject) -> bool {
    let (width_self, height_self) = object_1_read.dimensions();
    let (width_other, height_other) = object_2_read.dimensions();

    let half_width_self = width_self / 2.0;
    let half_height_self = height_self / 2.0;

    let half_width_other = width_other / 2.0;
    let half_height_other = height_other / 2.0;

    let object_1_pos = object_1_read.get_position();
    let object_2_pos = object_2_read.get_position();

    let self_min_x = object_1_pos.x - half_width_self;
    let self_max_x = object_1_pos.x + half_width_self;
    let self_min_y = object_1_pos.y - half_height_self;
    let self_max_y = object_1_pos.y + half_height_self;

    let other_min_x = object_2_pos.x - half_width_other;
    let other_max_x = object_2_pos.x + half_width_other;
    let other_min_y = object_2_pos.y - half_height_other;
    let other_max_y = object_2_pos.y + half_height_other;

    return self_min_x < other_max_x &&
    self_max_x > other_min_x &&
    self_min_y < other_max_y &&
    self_max_y > other_min_y;
}

fn is_colliding_circle(object_1_read: &Generic2DGraphicsObject,  object_2_read: &Generic2DGraphicsObject) -> bool {
    let object_1_pos = object_1_read.get_position();
    let object_2_pos = object_2_read.get_position();

    let dx = object_2_pos.x - object_1_pos.x;
    let dy = object_2_pos.y - object_1_pos.y;
    let distance_squared = dx * dx + dy * dy;

    let radius_self = object_1_read.get_radius();
    let radius_other = object_2_read.get_radius();

    let radius_sum = radius_self + radius_other;
    return distance_squared < radius_sum * radius_sum;
}

fn is_colliding_obb() -> bool {
    // Implement OBB collision logic here
    unimplemented!("OBB collision not yet implemented");
}  

// Check for collision with another object
// To have a collision, SELF and OTHER must SHARE the collision type. For example, Circle Collision objects cannot collide with AABB Collision Objects unless they both have AABB and Circle.
pub fn is_colliding(object_1_name: String,  object_2_name: String, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList) -> bool {
    if let Some(entity_1) = master_entity_list.get_entity(&object_1_name) {
        let entity_1_read = entity_1.read().unwrap();
        if let Some(object_2) = master_entity_list.get_entity(&object_2_name) {
            let entity_2_read = object_2.read().unwrap();
            let entity_1_collision_modes = entity_1_read.get_collision_modes();
            let entity_2_collision_modes = entity_2_read.get_collision_modes();

            if let Some(object_1) = master_graphics_list.get_object(&object_1_name) {
                let object_1_read = object_1.read().unwrap();
                if let Some(object_2) = master_graphics_list.get_object(&object_2_name) {
                    let object_2_read = object_2.read().unwrap();

                    for mode in entity_1_collision_modes {
                        if entity_2_collision_modes.contains(mode) && check_collision(&object_1_read, &object_2_read, *mode) {
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

// Helper to perform the appropriate collision check
fn check_collision(object_1_read: &Generic2DGraphicsObject, object_2_read: &Generic2DGraphicsObject, mode: CollisionMode) -> bool {
    match mode {
        CollisionMode::AABB => is_colliding_aabb(object_1_read, object_2_read),
        CollisionMode::Circle => is_colliding_circle(object_1_read, object_2_read),
        CollisionMode::OBB => is_colliding_obb(),
    }
}

pub fn collision_move_entity_based_on_position(master_graphics_list: &MasterGraphicsList, unmoved_entity: &GenericEntity, moved_entity: &GenericEntity, push_force: f32) {
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
