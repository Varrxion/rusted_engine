use std::sync::{Arc, RwLock};

use nalgebra::Vector3;
use rusted_open::framework::graphics::{internal_object::graphics_object::Generic2DGraphicsObject, util::master_graphics_list::MasterGraphicsList};
use rusted_open::framework::events::movement;

use crate::rusted_engine::entities::{generic_entity::{CollisionMode, GenericEntity}, util::master_entity_list::MasterEntityList};

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

fn check_collisions(master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, object_name: &str) -> Vec<CollisionEvent> {
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

fn is_colliding_aabb(object_1_read: &Generic2DGraphicsObject,  object_2_read: &Generic2DGraphicsObject) -> bool {
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
fn is_colliding(object_1_name: String,  object_2_name: String, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList) -> bool {
    if let Some(entity_1) = master_entity_list.get_entity(&object_1_name) {
        let entity_1_read = entity_1.read().unwrap();
        if let Some(object_2) = master_entity_list.get_entity(&object_2_name) {
            let entity_2_read = object_2.read().unwrap();
            let entity_1_collision_modes = entity_1_read.get_collision_modes();
            let entity_2_collision_modes = entity_2_read.get_collision_modes();

            if entity_1_read.get_collision_priority() >= entity_2_read.get_collision_priority() {
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

pub fn resolve_overlap(entity_1: &mut GenericEntity, entity_2: &mut GenericEntity, master_graphics_list: &MasterGraphicsList) {
    let object_1 = master_graphics_list.get_object(entity_1.get_name());
    let object_2 = master_graphics_list.get_object(entity_2.get_name());

    if let (Some(object_1), Some(object_2)) = (object_1, object_2) {
        let mut object_1 = object_1.write().unwrap();
        let mut object_2 = object_2.write().unwrap();

        let pos_1 = object_1.get_position();
        let pos_2 = object_2.get_position();
        let (width_1, height_1) = object_1.dimensions();
        let (width_2, height_2) = object_2.dimensions();

        let half_width_1 = width_1 * 0.5;
        let half_height_1 = height_1 * 0.5;
        let half_width_2 = width_2 * 0.5;
        let half_height_2 = height_2 * 0.5;

        let diff_x = pos_1.x - pos_2.x;
        let diff_y = pos_1.y - pos_2.y;

        let overlap_x = (half_width_1 + half_width_2) - diff_x.abs();
        let overlap_y = (half_height_1 + half_height_2) - diff_y.abs();

        if overlap_x > 0.0 && overlap_y > 0.0 {
            // Determine the axis of least penetration
            if overlap_x < overlap_y {
                // Resolve along the X-axis
                if diff_x > 0.0 {
                    object_1.set_position(Vector3::new(pos_2.x + half_width_2 + half_width_1, pos_1.y, pos_1.z));
                } else {
                    object_1.set_position(Vector3::new(pos_2.x - (half_width_2 + half_width_1), pos_1.y, pos_1.z));
                }
            } else {
                // Resolve along the Y-axis
                if diff_y > 0.0 {
                    object_1.set_position(Vector3::new(pos_1.x, pos_2.y + half_height_2 + half_height_1, pos_1.z));
                } else {
                    object_1.set_position(Vector3::new(pos_1.x, pos_2.y - (half_height_2 + half_height_1), pos_1.z));
                }
            }
        }
    } else {
        println!("One or both objects not found to resolve collision overlap");
    }
}


pub fn transfer_velocity_on_collision(entity_1: &mut GenericEntity, entity_2: &mut GenericEntity) {
    let velocity_1 = entity_1.get_velocity();
    let velocity_2 = entity_2.get_velocity();

    let weight_1 = entity_1.get_weight();
    let weight_2 = entity_2.get_weight();

    let elasticity_1 = entity_1.get_elasticity();
    let elasticity_2 = entity_2.get_elasticity();

    let is_entity_1_static = entity_1.is_static();
    let is_entity_2_static = entity_2.is_static();

    // Calculate the total mass (or weight)
    let total_weight = weight_1 + weight_2;

    // Calculate the velocity transfer between the two objects
    let velocity_diff = velocity_2 - velocity_1;

    // Apply the elasticity correction to the relative velocity change
    let velocity_transfer_1 = (velocity_diff * (2.0 * weight_2 / total_weight)) * elasticity_2;
    let velocity_transfer_2 = (-velocity_diff * (2.0 * weight_1 / total_weight)) * elasticity_1;

    // Only update the velocity for non-static entities
    if !is_entity_1_static {
        entity_1.set_velocity(velocity_1 + velocity_transfer_1);
    }

    if !is_entity_2_static {
        entity_2.set_velocity(velocity_2 + velocity_transfer_2);
    }
}

