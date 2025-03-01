use std::sync::{Arc, RwLock};

use glfw::Key;
use nalgebra::{Vector2, Vector3};
use rusted_open::framework::{events::movement, graphics::{internal_object::graphics_object::Generic2DGraphicsObject, util::master_graphics_list::MasterGraphicsList}};

use crate::rusted_engine::{entities::util::master_entity_list::MasterEntityList, input::key_states::KeyStates};

/// A more refined movement based on directional velocity.
pub fn process_object_acceleration(obj_name: String, normalize: bool, max_speed: f32, master_entity_list: &MasterEntityList, master_graphics_list: &MasterGraphicsList, key_states: Arc<RwLock<KeyStates>>, delta_time: f32) {
    if let Some(entity) = master_entity_list.get_entity(&obj_name) {
        if let Ok(mut entity) = entity.write() {
            if let Some(object) = master_graphics_list.get_object(&obj_name) {
                let mut acceleration = Vector2::new(0.0, 0.0);

                let key_states_read = key_states.read().unwrap();
                if key_states_read.is_key_pressed_raw(Key::W) {
                    acceleration.y += 1.0;
                }
                if key_states_read.is_key_pressed_raw(Key::S) {
                    acceleration.y -= 1.0;
                }
                if key_states_read.is_key_pressed_raw(Key::A) {
                    acceleration.x -= 1.0;
                }
                if key_states_read.is_key_pressed_raw(Key::D) {
                    acceleration.x += 1.0;
                }

                // Normalize the acceleration vector to prevent faster diagonal movement
                if normalize == true {
                    if acceleration.magnitude() > 0.0 {
                        acceleration = acceleration.normalize();
                    }
                }

                // Apply acceleration to the entity's velocity
                let move_speed = 1.1;
                let new_velocity = entity.get_velocity() + acceleration * move_speed * delta_time;

                entity.set_velocity(new_velocity);

                let mut velocity = entity.get_velocity();
                let current_speed = velocity.magnitude();

                // If the current speed exceeds the max speed, normalize and scale it
                if current_speed > max_speed {
                    velocity = velocity.normalize() * max_speed;
                    entity.set_velocity(velocity); // Set the capped velocity back to the entity
                }

                movement::move_object(object.clone(), Vector3::new(velocity.x, velocity.y, 0.0), move_speed, delta_time);
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

/// A method to apply friction to the object’s movement when no directional keys are pressed or when a change in direction occurs.
pub fn process_object_fake_friction(obj_name: String, vertical_friction: bool, master_entity_list: &MasterEntityList, key_states: Arc<RwLock<KeyStates>>, delta_time: f32) {
    if let Some(entity) = master_entity_list.get_entity(&obj_name) {
        if let Ok(mut entity) = entity.write() {
            // Define the friction value
            let friction = 2.5;

            let mut velocity = entity.get_velocity();

            let key_states_read = key_states.read().unwrap();
            let is_left_pressed = key_states_read.is_key_pressed_raw(Key::A);
            let is_right_pressed = key_states_read.is_key_pressed_raw(Key::D);
            let is_up_pressed = key_states_read.is_key_pressed_raw(Key::W);
            let is_down_pressed = key_states_read.is_key_pressed_raw(Key::S);

            // Apply horizontal friction based on the velocity and key presses
            if !is_left_pressed && !is_right_pressed {
                // No keys pressed, apply friction in the direction of velocity horizontally
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
            } else {
                // When turning direction (holding opposite keys), apply friction in the opposite direction of velocity horizontally
                if is_left_pressed && velocity.x > 0.0 {
                    velocity.x -= friction * delta_time; // Apply leftward friction
                } else if is_right_pressed && velocity.x < 0.0 {
                    velocity.x += friction * delta_time; // Apply rightward friction
                }
            }

            // If vertical friction is enabled, apply similar logic to vertical movement
            if vertical_friction {
                if !is_up_pressed && !is_down_pressed {
                    // No keys pressed vertically, apply friction in the direction of vertical velocity
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
                } else {
                    // When turning direction (holding opposite vertical keys), apply friction in the opposite direction of velocity vertically
                    if !is_up_pressed && velocity.y > 0.0 {
                        velocity.y -= friction * delta_time; // Apply upward friction
                    } else if !is_down_pressed && velocity.y < 0.0 {
                        velocity.y += friction * delta_time; // Apply downward friction
                    }
                }
            }

            // Set the new velocity with the applied friction
            entity.set_velocity(velocity);
        }
        else {
            println!("Couldn't acquire a write lock on that entity. Cannot process object friction");
        }
    }
    else {
        println!("No entity with that name could be found. Cannot process object friction");
    }
}



/// Raw, Direct graphics object movement based on active keys
pub fn process_object_raw_movement(controlled_obj: Arc<RwLock<Generic2DGraphicsObject>>, key_states: Arc<RwLock<KeyStates>>, delta_time: f32) {
    let move_speed = 0.2;
    let rotation_speed = 2.0;
    let key_states_read = key_states.read().unwrap();

    if key_states_read.is_key_pressed_raw(Key::W) {
        movement::move_object(controlled_obj.clone(), Vector3::new(0.0, 1.0, 0.0), move_speed, delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::S) {
        movement::move_object(controlled_obj.clone(), Vector3::new(0.0, -1.0, 0.0), move_speed, delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::A) {
        movement::move_object(controlled_obj.clone(), Vector3::new(-1.0, 0.0, 0.0), move_speed, delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::D) {
        movement::move_object(controlled_obj.clone(), Vector3::new(1.0, 0.0, 0.0), move_speed, delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::Q) {
        movement::rotate_object(controlled_obj.clone(), rotation_speed*delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::E) {
        movement::rotate_object(controlled_obj.clone(), -rotation_speed*delta_time);
    }
    if key_states_read.is_key_pressed_raw(Key::G) {
        controlled_obj.write().unwrap().set_rotation(0.0);
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
