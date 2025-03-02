use std::collections::HashSet;

use nalgebra::Vector2;

pub struct GenericEntity {
    name: String,
    weight: f32,
    velocity: Vector2<f32>,
    affected_by_gravity: bool,
    is_static: bool,
    elasticity: f32,
    can_destroy: bool,
    destructible: bool,
    active_collision: bool,
    collision_priority: u64,
    collision_modes: HashSet<CollisionMode>,
    collision_sound: String,
}

impl Clone for GenericEntity {
    fn clone(&self) -> Self {
        GenericEntity {
            name: self.name.clone(),
            weight: self.weight,
            velocity: self.velocity,
            affected_by_gravity: self.affected_by_gravity,
            is_static: self.is_static,
            elasticity: self.elasticity,
            can_destroy: self.can_destroy,
            destructible: self.destructible,
            active_collision: self.active_collision,
            collision_priority: self.collision_priority,
            collision_modes: self.collision_modes.clone(),
            collision_sound: self.collision_sound.clone(),
        }
    }
}

impl GenericEntity {
    pub fn new(name: String, weight: f32, velocity: Vector2<f32>, affected_by_gravity: bool, is_static: bool, elasticity: f32, can_destroy: bool, destructible: bool, active_collision: bool, collision_priority: u64, collision_modes: HashSet<CollisionMode>, collision_sound: String) -> Self {
        GenericEntity {
            name,
            weight,
            velocity,
            affected_by_gravity,
            is_static,
            elasticity,
            can_destroy,
            destructible,
            active_collision,
            collision_priority,
            collision_modes,
            collision_sound,
        }
    }

    // Getters
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_weight(&self) -> f32 {
        self.weight
    }

    pub fn get_velocity(&self) -> Vector2<f32> {
        self.velocity
    }

    pub fn is_affected_by_gravity(&self) -> bool {
        self.affected_by_gravity
    }

    pub fn is_static(&self) -> bool {
        self.is_static
    }

    pub fn get_elasticity(&self) -> f32 {
        self.elasticity
    }

    pub fn can_destroy(&self) -> bool {
        self.can_destroy
    }

    pub fn is_destructible(&self) -> bool {
        self.destructible
    }

    pub fn has_active_collision(&self) -> bool {
        self.active_collision
    }

    pub fn get_collision_priority(&self) -> u64 {
        self.collision_priority
    }

    pub fn get_collision_modes(&self) -> &HashSet<CollisionMode> {
        &self.collision_modes
    }

    pub fn get_collision_sound(&self) -> &str {
        &self.collision_sound
    }

    // Setters
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_weight(&mut self, weight: f32) {
        self.weight = weight;
    }

    pub fn set_velocity(&mut self, velocity: Vector2<f32>) {
        self.velocity = velocity;
    }

    pub fn set_affected_by_gravity(&mut self, affected_by_gravity: bool) {
        self.affected_by_gravity = affected_by_gravity;
    }

    pub fn set_is_static(&mut self, is_static: bool) {
        self.is_static = is_static;
    }

    pub fn set_elasticity(&mut self, elasticity: f32) {
        self.elasticity = elasticity;
    }

    pub fn set_can_destroy(&mut self, can_destroy: bool) {
        self.can_destroy = can_destroy;
    }

    pub fn set_destructible(&mut self, destructible: bool) {
        self.destructible = destructible;
    }

    pub fn set_active_collision(&mut self, active_collision: bool) {
        self.active_collision = active_collision;
    }

    pub fn set_collision_priority(&mut self, collision_priority: u64) {
        self.collision_priority = collision_priority;
    }

    pub fn set_collision_modes(&mut self, modes: HashSet<CollisionMode>) {
        self.collision_modes = modes;
    }

    pub fn set_collision_sound(&mut self, collision_sound: String) {
        self.collision_sound = collision_sound;
    }

    // Game Logic

    pub fn apply_gravity(&mut self, gravity: Vector2<f32>, terminal_velocity: Vector2<f32>, delta_time: f32) {
    if self.affected_by_gravity {
        // Apply gravity to the velocity
        self.velocity += gravity * delta_time;
        
        // Clamp the velocity to the terminal velocity
        self.velocity.x = self.velocity.x.clamp(-terminal_velocity.x, terminal_velocity.x);
        self.velocity.y = self.velocity.y.clamp(-terminal_velocity.y, terminal_velocity.y);
    }
}


    // Debug

    pub fn print_debug(&self) {
        println!("Debug Info for GenericEntity:");
        println!("Name: {}", self.name);
        println!("Weight: {}", self.weight);
        println!("Velocity: {}", self.velocity);
        println!("Affected by Gravity: {}", self.affected_by_gravity);
        println!("Can Destroy: {}", self.can_destroy);
        println!("Destructible: {}", self.destructible);
        println!("Active Collision: {}", self.active_collision);
        println!("Collision mode(s): {:?}", self.collision_modes)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollisionMode {
    AABB,
    Circle,
    OBB,
}