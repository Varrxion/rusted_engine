use std::sync::{Arc, RwLock};

use glfw::Key;
use nalgebra::{Vector2, Vector3};
use rusted_open::framework::{events::movement, graphics::{internal_object::graphics_object::Generic2DGraphicsObject, util::master_graphics_list::MasterGraphicsList}};

use crate::rusted_engine::{entities::util::master_entity_list::MasterEntityList, input::key_states::KeyStates};

/// A more refined movement based on directional velocity.
pub fn process_object_acceleration(obj_name: String, normalize: bool, speed: f32, max_speed: f32, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, key_states: Arc<RwLock<KeyStates>>, delta_time: f32) {
    if let Some(entity) = master_entity_list.get_entity(&obj_name) {
        if let Ok(mut entity) = entity.write() {
            if let Some(object) = master_graphics_list.get_object(&obj_name) {
                let mut acceleration = Vector2::new(0.0, 0.0);

                let key_states_read = key_states.read().unwrap();
                if key_states_read.is_key_pressed_raw(Key::W) {
                    acceleration.y += speed;
                }
                if key_states_read.is_key_pressed_raw(Key::S) {
                    acceleration.y -= speed;
                }
                if key_states_read.is_key_pressed_raw(Key::A) {
                    acceleration.x -= speed;
                }
                if key_states_read.is_key_pressed_raw(Key::D) {
                    acceleration.x += speed;
                }

                // Normalize the acceleration vector to prevent faster diagonal movement
                if normalize == true {
                    if acceleration.magnitude() > 0.0 {
                        acceleration = acceleration.normalize();
                    }
                }

                // Apply acceleration to the entity's velocity
                let new_velocity = entity.get_velocity() + acceleration * delta_time;

                entity.set_velocity(new_velocity);

                let mut velocity = entity.get_velocity();
                let current_speed = velocity.magnitude();

                // If the current speed exceeds the max speed, normalize and scale it
                if current_speed > max_speed {
                    velocity = velocity.normalize() * max_speed;
                    entity.set_velocity(velocity); // Set the capped velocity back to the entity
                }

                //movement::move_object(&mut object.write().unwrap().clone(), Vector3::new(velocity.x, velocity.y, 0.0), delta_time);
            }
            else {
                println!("No object with that name could be found. Cannot process object acceleration");
            }
        }
        else {
            println!("Couldn't acquire a read lock on that entity. Cannot process object acceleration");
        }
    }
    else {
        println!("No entity with that name could be found. Cannot process object acceleration");
    }
}

pub fn process_object_friction() {
    // TBD After the collision prediction system
    // We can slow the object based on a percentage of total velocity + a small base value to prevent too slow of friction at low speeds.
}

/// A method to apply friction to all object movement, for use in top-down view or zero-gravity environment
pub fn process_all_entities_fake_friction(friction: f32, master_entity_list: &MasterEntityList, vertical_friction: bool, delta_time: f32) {
    // Iterate through all entities in the master entity list
    let entities = master_entity_list.get_entities();
    let entities = entities.read().unwrap();

    for (entity_name, entity_ref) in entities.iter() {
        if let Ok(mut entity) = entity_ref.write() {
            let mut velocity = entity.get_velocity();

            // Apply horizontal friction based on the velocity
            if velocity.x > 0.0 {
                velocity.x -= friction * delta_time;
                if velocity.x < 0.0 {
                    velocity.x = 0.0;
                }
            } else if velocity.x < 0.0 {
                velocity.x += friction * delta_time;
                if velocity.x > 0.0 {
                    velocity.x = 0.0;
                }
            }

            // Apply vertical friction if enabled (based on argument)
            if vertical_friction {
                if velocity.y > 0.0 {
                    velocity.y -= friction * delta_time;
                    if velocity.y < 0.0 {
                        velocity.y = 0.0;
                    }
                } else if velocity.y < 0.0 {
                    velocity.y += friction * delta_time;
                    if velocity.y > 0.0 {
                        velocity.y = 0.0;
                    }
                }
            }

            // Set the new velocity with the applied friction
            entity.set_velocity(velocity);
        } else {
            println!("Couldn't acquire a write lock on entity: {}. Cannot process friction.", entity_name);
        }
    }
}




/// Raw, Direct graphics object movement based on active keys
pub fn process_object_raw_movement(controlled_obj: Arc<RwLock<Generic2DGraphicsObject>>, key_states: Arc<RwLock<KeyStates>>, delta_time: f32) {
    let rotation_speed = 2.0;
    let key_states_read = key_states.read().unwrap();

    let mut controlled_obj = controlled_obj.write().unwrap();

    if key_states_read.is_key_pressed_raw(Key::W) {
        movement::move_object(&mut controlled_obj, Vector3::new(0.0, 1.0, 0.0), delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::S) {
        movement::move_object(&mut controlled_obj, Vector3::new(0.0, -1.0, 0.0), delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::A) {
        movement::move_object(&mut controlled_obj, Vector3::new(-1.0, 0.0, 0.0), delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::D) {
        movement::move_object(&mut controlled_obj, Vector3::new(1.0, 0.0, 0.0), delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::Q) {
        movement::rotate_object(&mut controlled_obj, rotation_speed*delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::E) {
        movement::rotate_object(&mut controlled_obj, -rotation_speed*delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::G) {
        controlled_obj.set_rotation(0.0);
    }
}

pub fn gravity(gravity: Vector2<f32>, terminal_velocity: Vector2<f32>, master_entity_list: &MasterEntityList, delta_time: f32) {
    let entities = master_entity_list.get_entities();
    let entities_read = entities.read().unwrap();
    for entity in entities_read.values() {
        if let Ok(mut entity) = entity.write() {
            entity.apply_gravity(gravity, terminal_velocity, delta_time);
        }
    }
}

pub fn process_movement(master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, delta_time: f32) {
    let entities = master_entity_list.get_entities();
    let entities_read = entities.read().unwrap();
    for entity in entities_read.values() {
        if let Ok(entity) = entity.write() {
            // Retrieve the corresponding graphics object
            if let Some(entity_graphics_object) = master_graphics_list.get_object(&entity.get_name()) {
                if let Ok(mut graphics_object) = entity_graphics_object.write() { // Acquire a write lock on the graphics object

                    // Get the current position and velocity from the entity and graphics
                    let velocity = entity.get_velocity();

                    // Update the position based on velocity and the passed delta_time
                    movement::move_object(&mut graphics_object, Vector3::new(velocity.x, velocity.y, 0.0), delta_time);
                }
            }
        }
    }
}