pub struct GenericEntity {
    name: String,
    weight: f32,
    can_destroy: bool,
    destructible: bool,
    active_collision: bool,
}

impl GenericEntity {
    pub fn new(name: String, weight: f32, can_destroy: bool, destructible: bool, active_collision: bool) -> Self {
        GenericEntity {
            name,
            weight,
            can_destroy,
            destructible,
            active_collision,
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

    pub fn print_debug(&self) {
        println!("Debug Info for GenericEntity:");
        println!("Name: {}", self.name);
        println!("Weight: {}", self.weight);
        println!("Can Destroy: {}", self.can_destroy);
        println!("Destructible: {}", self.destructible);
        println!("Active Collision: {}", self.active_collision);
    }
}