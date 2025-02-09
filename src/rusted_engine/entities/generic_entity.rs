use std::collections::HashSet;

pub struct GenericEntity {
    name: String,
    weight: f32,
    can_destroy: bool,
    destructible: bool,
    active_collision: bool,
    collision_modes: HashSet<CollisionMode>,
    collision_sound: String,
}

impl Clone for GenericEntity {
    fn clone(&self) -> Self {
        GenericEntity {
            name: self.name.clone(),
            weight: self.weight,
            can_destroy: self.can_destroy,
            destructible: self.destructible,
            active_collision: self.active_collision,
            collision_modes: self.collision_modes.clone(),
            collision_sound: self.collision_sound.clone(),
        }
    }
}

impl GenericEntity {
    pub fn new(name: String, weight: f32, can_destroy: bool, destructible: bool, active_collision: bool, collision_modes: HashSet<CollisionMode>, collision_sound: String) -> Self {
        GenericEntity {
            name,
            weight,
            can_destroy,
            destructible,
            active_collision,
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

    pub fn can_destroy(&self) -> bool {
        self.can_destroy
    }

    pub fn is_destructible(&self) -> bool {
        self.destructible
    }

    pub fn has_active_collision(&self) -> bool {
        self.active_collision
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

    pub fn set_can_destroy(&mut self, can_destroy: bool) {
        self.can_destroy = can_destroy;
    }

    pub fn set_destructible(&mut self, destructible: bool) {
        self.destructible = destructible;
    }

    pub fn set_active_collision(&mut self, active_collision: bool) {
        self.active_collision = active_collision;
    }

    pub fn set_collision_modes(&mut self, modes: HashSet<CollisionMode>) {
        self.collision_modes = modes;
    }

    pub fn set_collision_sound(&mut self, collision_sound: String) {
        self.collision_sound = collision_sound;
    }

    pub fn print_debug(&self) {
        println!("Debug Info for GenericEntity:");
        println!("Name: {}", self.name);
        println!("Weight: {}", self.weight);
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